use crate::buffer::Buffer;
use crate::hardware::Hardware;
use core::mem::MaybeUninit;
use usb_device::prelude::*;
use usb_device::class_prelude::*;
use usb_device::Result;
use usb_device::device::UsbDevice;

const SIO_RESET_REQUEST: u8 = 0x00;
const SIO_SET_MODEM_CTRL_REQUEST: u8 = 0x01;
const SIO_SET_FLOW_CTRL_REQUEST: u8 = 0x02;
const SIO_SET_BAUDRATE_REQUEST: u8 = 0x03;
const SIO_SET_DATA_REQUEST: u8 = 0x04;
const SIO_POLL_MODEM_STATUS_REQUEST: u8 = 0x05;
const SIO_SET_EVENT_CHAR_REQUEST: u8 = 0x06;
const SIO_SET_ERROR_CHAR_REQUEST: u8 = 0x07;
const SIO_SET_LATENCY_TIMER_REQUEST: u8 = 0x09;
const SIO_GET_LATENCY_TIMER_REQUEST: u8 = 0x0A;
const SIO_SET_BITMODE_REQUEST: u8 = 0x0B;
const SIO_READ_PINS_REQUEST: u8 = 0x0C;
// const SIO_READ_EEPROM_REQUEST: u8 = 0x90;
// const SIO_WRITE_EEPROM_REQUEST: u8 = 0x91;
// const SIO_ERASE_EEPROM_REQUEST: u8 = 0x92;

const SIO_RESET_SIO: u8 = 0;
const SIO_RESET_PURGE_RX: u8 = 1;
const SIO_RESET_PURGE_TX: u8 = 2;

const BITMODE_RESET: u8 = 0;
const BITMODE_MPSSE: u8 = 2;

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum FtdiMode {
    Serial,
    MPSSE,
}

pub struct FtdiPort<'a, B: UsbBus> {
    interface: InterfaceNumber,
    read_ep: EndpointOut<'a, B>,
    write_ep: EndpointIn<'a, B>,
    fixed_mode: FtdiMode,
    current_mode: FtdiMode,
    latency_timer: u8,
    rx_buffer: Buffer<[u8; 384]>,
    tx_buffer: Buffer<[u8; 128]>,
    hardware: &'a Hardware,
}

impl<'a, B: UsbBus> FtdiPort<'a, B> {
    pub fn new<'b: 'a>(
        alloc: &'b UsbBusAllocator<B>,
        hardware: &'a Hardware,
        fixed_mode: FtdiMode,
    ) -> FtdiPort<'a, B> {
        let interface = alloc.interface();

        let read_ep_addr = EndpointAddress::from(0x02 + u8::from(interface) * 2);
        let write_ep_addr = EndpointAddress::from(0x81 + u8::from(interface) * 2);

        let read_ep = alloc
            .alloc(Some(read_ep_addr), EndpointType::Bulk, 64, 0)
            .expect("alloc_ep failed");
        let write_ep = alloc
            .alloc(Some(write_ep_addr), EndpointType::Bulk, 64, 0)
            .expect("alloc_ep failed");

        let rx_buffer: [u8; 384] = unsafe { MaybeUninit::uninit().assume_init() };
        let tx_buffer: [u8; 128] = unsafe { MaybeUninit::uninit().assume_init() };

        FtdiPort {
            interface,
            read_ep,
            write_ep,
            fixed_mode,
            current_mode: fixed_mode,
            latency_timer: 16,
            rx_buffer: Buffer::new(rx_buffer),
            tx_buffer: Buffer::new(tx_buffer),
            hardware,
        }
    }

    pub fn make_device(usb_bus: &'a UsbBusAllocator<B>) -> UsbDevice<'a, B> {
        UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x0403, 0x6010))
            .manufacturer("FTDI")
            .product("Dual RS232")
            .serial_number("Nope, it's STM32F042")
            .device_release(0x0500)
            .build()
    }

    fn process_mpsse_command(hardware: &Hardware, data: &[u8], tx_buffer: &mut Buffer<[u8; 128]>) -> usize {
        if data.is_empty() {
            return 0;
        }

        let command = data[0];
        match command {
            0x1a => {
                // Clock Data Bits Out on +ve clock edge LSB first (no read)
                if data.len() < 3 {
                    return 0;
                }
                // Not supported
                3
            }
            0x1b => {
                // Clock Data Bits Out on -ve clock edge LSB first (no read)
                if data.len() < 3 {
                    return 0;
                }
                let nbits = (data[1] & 7) + 1;
                let byte = data[2];
                hardware.mpsse_write_tdi_bits_lsb_mode0(byte, nbits);
                3
            }
            0x39 | 0x3c => {
                // Clock Data Bytes In and Out LSB first
                if data.len() < 4 {
                    return 0;
                }
                let n = (data[1] as u16) | ((data[2] as u16) << 8);
                let nbytes = (n as usize) + 1;
                if data.len() < (3 + nbytes) {
                    return 0;
                }

                if tx_buffer.available_write() < nbytes {
                    return 0;
                }

                if command == 0x39 {
                    tx_buffer.write_all(nbytes, |buffer| {
                        hardware.mpsse_transfer_tdi_bytes_lsb_mode0(&data[2..2+nbytes], buffer);
                        Result::Ok(nbytes)
                    }).ok();
                } else {
                    // Not supported
                    tx_buffer.write_all(nbytes, |buffer| {
                        Result::Ok(nbytes)
                    }).ok();
                }

                3 + nbytes
            }
            0x3b | 0x3e => {
                // Clock Data Bits In and Out LSB first
                if data.len() < 3 {
                    return 0;
                }

                if tx_buffer.available_write() < 1 {
                    return 0;
                }

                let response;
                if command == 0x3B {
                    let nbits = (data[1] & 7) + 1;
                    let byte = data[2];
                    response = hardware.mpsse_transfer_tdi_bits_lsb_mode0(byte, nbits);
                } else {
                    response = 0; // Not supported
                }

                tx_buffer.write_all(1, |buffer| {
                    buffer[0] = response;
                    Result::Ok(1)
                }).ok();

                3
            }
            0x4a | 0x4b => {
                // Clock Data to TMS pin (no read)
                if data.len() < 3 {
                    return 0;
                }
                if command == 0x4b {
                    let nbits = (data[1] & 7) + 1;
                    let byte = data[2];
                    hardware.mpsse_write_tms_bits_lsb_mode0(byte, nbits);
                }
                3
            }
            0x6a | 0x6b | 0x6e | 0x6f => {
                // Clock Data to TMS pin with read
                if data.len() < 3 {
                    return 0;
                }
                if tx_buffer.available_write() < 1 {
                    return 0;
                }

                let response;
                if command == 0x6b {
                    let nbits = (data[1] & 7) + 1;
                    let byte = data[2];
                    response = hardware.mpsse_transfer_tms_bits_lsb_mode0(byte, nbits);
                } else {
                    response = 0; // Not supported
                }

                tx_buffer.write_all(1, |buffer| {
                    buffer[0] = response;
                    Result::Ok(1)
                }).ok();

                3
            }
            0x80 => {
                // Set Data bits LowByte
                if data.len() < 3 {
                    return 0;
                }
                let value = data[1];
                let direction = data[2];
                hardware.mpsse_set_gpio_value(direction, value);
                3
            }
            0x82 => {
                // Set Data bits High Byte
                if data.len() < 3 {
                    return 0;
                }
                // High bytes are not supported, discard
                3
            }
            0x84 => {
                // Connect TDI to TDO for Loopback

                // Not supported
                1
            }
            0x85 => {
                // Disconnect TDI to TDO for Loopback

                // Not supported
                1
            }
            0x86 => {
                // Set TCK/SK Divisor
                if data.len() < 3 {
                    return 0;
                }

                let div = (data[1] as u16) | ((data[2] as u16) << 8);
                let div = div as u32 + 1;
                hardware.mpsse_set_tck_divisor(div);
                3
            }
            0x87 => {
                // Send Immediate
                // Not supported
                1
            }
            _ => {
                // Unknown command, hang
                0
            }
        }
    }

    pub fn process_commands(&mut self) {
        self.process_rx();

        if self.current_mode != self.fixed_mode {
            return;
        }

        match self.current_mode {
            FtdiMode::Serial => {
                // TODO: implement serial

                // Loopback all the data for now
                let rx_buffer = &mut self.rx_buffer;
                let tx_buffer = &mut self.tx_buffer;
                if rx_buffer.available_read() > 0 && tx_buffer.available_write() > 0 {
                    let n = core::cmp::min(rx_buffer.available_read(), tx_buffer.available_write());
                    tx_buffer.write_all(n, |buffer| {
                        rx_buffer.read(n, |data| {
                            buffer.copy_from_slice(data);
                            Result::Ok(data.len())
                        })
                    }).ok();
                } else if rx_buffer.available_read() > 0 && tx_buffer.available_write() == 0 {
                    // Discard all the data
                    rx_buffer.clear();
                }
            }
            FtdiMode::MPSSE => {
                let hardware = self.hardware;
                let tx_buffer = &mut self.tx_buffer;
                while self.rx_buffer.available_read() > 0 {
                    let n = self.rx_buffer.read(usize::MAX, |buffer| {
                        let n = Self::process_mpsse_command(hardware, buffer, tx_buffer);
                        Result::Ok(n)
                    }).unwrap();

                    if n == 0 {
                        break;
                    }
                }
            }
        }

        self.process_tx();
    }

    fn process_rx(&mut self) {
        if self.current_mode != self.fixed_mode {
            // Discard incoming packet
            let mut buf: [u8; 64] = unsafe { MaybeUninit::uninit().assume_init() };
            self.read_ep.read(&mut buf).ok();

            return;
        }

        // Try to read from the endpoint.
        // Nothing happens on error or if there is no space in the buffer.
        let rx_buffer = &mut self.rx_buffer;
        let read_ep = &mut self.read_ep;
        rx_buffer.write_all(read_ep.max_packet_size() as usize, |buf| {
            match read_ep.read(buf) {
                Ok(n) => Ok(n),
                Err(UsbError::WouldBlock) => Ok(0),
                Err(err) => Err(err),
            }
        }).ok();
    }

    fn process_tx(&mut self) {
        let tx_buffer = &mut self.tx_buffer;
        let write_ep = &mut self.write_ep;

        match self.current_mode {
            FtdiMode::Serial => {
                if tx_buffer.available_read() > 0 {
                    let mut buf: [u8; 64] = unsafe { MaybeUninit::uninit().assume_init() };
                    buf[0] = 0x01;
                    buf[1] = 0x60;
                    tx_buffer.read(62, |bytes| {
                        // `bytes` slice is max 62 bytes, so copy everything we got
                        buf[2..2 + bytes.len()].copy_from_slice(bytes);

                        write_ep.write(&buf[..2 + bytes.len()]).map(|n| n - 2)
                    }).ok();
                } else {
                    write_ep.write(&[0x01, 0x60]).ok();
                }
            }
            FtdiMode::MPSSE => {
                if tx_buffer.available_read() > 0 {
                    tx_buffer.read(write_ep.max_packet_size() as usize, |bytes| {
                        write_ep.write(bytes)
                    }).ok();
                }
            }
        }
    }
}

impl<B: UsbBus> UsbClass<B> for FtdiPort<'_, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        writer.interface(self.interface, 0xff, 0xff, 0xff)?;

        writer.endpoint(&self.write_ep)?;
        writer.endpoint(&self.read_ep)?;

        Ok(())
    }

    fn reset(&mut self) {
        self.latency_timer = 16;

        self.rx_buffer.clear();
        self.tx_buffer.clear();
    }

    fn control_out(&mut self, xfer: ControlOut<B>) {
        let req = xfer.request();

        let port_index = req.index as u8;
        if !(req.request_type == control::RequestType::Vendor
            && req.recipient == control::Recipient::Device
            && port_index == (u8::from(self.interface) + 1))
        {
            return;
        }

        match req.request {
            SIO_RESET_REQUEST => {
                let reset_type = req.value as u8;
                match reset_type {
                    SIO_RESET_SIO => {
                        self.rx_buffer.clear();
                        self.tx_buffer.clear();
                    }
                    SIO_RESET_PURGE_RX => {
                        self.rx_buffer.clear();
                    }
                    SIO_RESET_PURGE_TX => {
                        self.tx_buffer.clear();
                    }
                    _ => {}
                }

                xfer.accept().ok();
            }
            SIO_SET_MODEM_CTRL_REQUEST => {
                // Ignore request
                xfer.accept().ok();
            }
            SIO_SET_FLOW_CTRL_REQUEST => {
                // Ignore request
                xfer.accept().ok();
            }
            SIO_SET_BAUDRATE_REQUEST => {
                // TODO: process request
                xfer.accept().ok();
            }
            SIO_SET_DATA_REQUEST => {
                let data_bits = req.value as u8;
                let params = (req.value >> 8) as u8;
                // TODO: process request
                xfer.accept().ok();
            }
            SIO_SET_EVENT_CHAR_REQUEST => {
                // Ignore request
                xfer.accept().ok();
            }
            SIO_SET_ERROR_CHAR_REQUEST => {
                // Ignore request
                xfer.accept().ok();
            }
            SIO_SET_LATENCY_TIMER_REQUEST => {
                self.latency_timer = req.value as u8;
                xfer.accept().ok();
            }
            SIO_SET_BITMODE_REQUEST => {
                let _bit_mask = req.value as u8;
                let bit_mode = (req.value >> 8) as u8;

                let new_mode = match bit_mode {
                    BITMODE_RESET => FtdiMode::Serial,
                    BITMODE_MPSSE => FtdiMode::MPSSE,
                    _ => FtdiMode::Serial,
                };
                self.current_mode = new_mode;

                // TODO: process bitmask

                xfer.accept().ok();
            }
            _ => {
                xfer.reject().ok();
            }
        }
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = xfer.request();

        let port_index = req.index as u8;
        if !(req.request_type == control::RequestType::Vendor
            && req.recipient == control::Recipient::Device
            && port_index == (u8::from(self.interface) + 1))
        {
            return;
        }

        match req.request {
            SIO_GET_LATENCY_TIMER_REQUEST if req.length == 1 => {
                xfer.accept(|data| {
                    data[0] = self.latency_timer;

                    Ok(1)
                })
                .ok();
            }
            SIO_POLL_MODEM_STATUS_REQUEST if req.length == 2 => {
                xfer.accept(|data| {
                    data[0] = 0x01;
                    data[1] = 0x60;

                    Ok(2)
                })
                .ok();
            }
            SIO_READ_PINS_REQUEST if req.length == 1 => {
                // TODO: process request
                xfer.accept(|data| {
                    data[0] = 0x00;

                    Ok(1)
                })
                .ok();
            }
            _ => {
                xfer.reject().ok();
            }
        }
    }

    fn endpoint_out(&mut self, addr: EndpointAddress) {
        if addr != self.read_ep.address() {
            self.process_rx();
        }
    }

    fn endpoint_in_complete(&mut self, addr: EndpointAddress) {
        if addr == self.write_ep.address() {
            self.process_tx();
        }
    }
}
