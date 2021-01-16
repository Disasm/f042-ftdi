use core::sync::atomic::{AtomicU32, Ordering};
use stm32f0xx_hal::pac;

pub struct Hardware {
    mpsse_divisor: AtomicU32,
}

impl Hardware {
    pub fn new() -> Self {
        Hardware {
            mpsse_divisor: AtomicU32::new(1),
        }
    }

    pub fn set_gpio_direction_mpsse(&self, direction: u8) {
        unsafe {
            let gpio = &*pac::GPIOA::ptr();
            gpio.moder.modify(|_, w| {
                if direction & 0b0000_0001 != 0 {
                    w.moder5().output();
                } else {
                    w.moder5().input();
                }

                if direction & 0b0000_0010 != 0 {
                    w.moder7().output();
                } else {
                    w.moder7().input();
                }

                if direction & 0b0000_0100 != 0 {
                    w.moder6().output();
                } else {
                    w.moder6().input();
                }

                if direction & 0b0000_1000 != 0 {
                    w.moder4().output();
                } else {
                    w.moder4().input();
                }

                w
            });
        }
    }

    pub fn set_gpio_direction_serial(&self, direction: u8) {
        let _ = direction;
    }

    pub fn mpsse_set_gpio_value(&self, direction: u8, value: u8) {
        self.set_gpio_direction_mpsse(direction);

        unsafe {
            let gpio = &*pac::GPIOA::ptr();
            gpio.bsrr.write(|w| {
                if value & 0b0000_0001 != 0 {
                    w.bs5().set_bit();
                } else {
                    w.br5().set_bit();
                }

                if value & 0b0000_0010 != 0 {
                    w.bs7().set_bit();
                } else {
                    w.br7().set_bit();
                }

                if value & 0b0000_0100 != 0 {
                    w.bs6().set_bit();
                } else {
                    w.br6().set_bit();
                }

                if value & 0b0000_1000 != 0 {
                    w.bs4().set_bit();
                } else {
                    w.br4().set_bit();
                }

                w
            });
        }
    }

    // Set `divisor` for 6MHz clock
    pub fn mpsse_set_tck_divisor(&self, divisor: u32) {
        self.mpsse_divisor.store(divisor, Ordering::SeqCst);
    }

    // [0x1B] Clock Data Bits Out on -ve clock edge LSB first (no read)
    // CLK starts at 0
    pub fn mpsse_write_tdi_bits_lsb_mode0(&self, byte: u8, nbits: u8) {
        let div = self.mpsse_divisor.load(Ordering::SeqCst);

        extern "C" {
            fn _write_tdi_bits_lsb_mode0_6mhz(byte: u8, nbits: u8, bsrr: *mut u32);
            fn _write_tdi_bits_lsb_mode0_3mhz(byte: u8, nbits: u8, bsrr: *mut u32);
            fn _write_tdi_bits_lsb_mode0_delay(byte: u8, nbits: u8, bsrr: *mut u32, delay: u32);
        }

        unsafe {
            let gpio = &*pac::GPIOA::ptr();
            let bsrr = &gpio.bsrr as *const _ as *mut u32;

            match div {
                0 => {},
                1 => _write_tdi_bits_lsb_mode0_6mhz(byte, nbits, bsrr),
                2 => _write_tdi_bits_lsb_mode0_3mhz(byte, nbits, bsrr),
                n => _write_tdi_bits_lsb_mode0_delay(byte, nbits, bsrr, n - 3)
            }
        }
    }

    // [0x39] Clock Data Bytes In and Out LSB first
    // Out on -ve edge, in on +ve edge
    pub fn mpsse_transfer_tdi_bytes_lsb_mode0(&self, tdi_bytes: &[u8], tdo_bytes: &mut [u8]) {
        // TODO: improve
        for (tdi, tdo) in tdi_bytes.iter().zip(tdo_bytes.iter_mut()) {
            *tdo = self.mpsse_transfer_tdi_bits_lsb_mode0(*tdi, 8);
        }
    }

    // [0x3B] Clock Data Bits In and Out LSB first
    // Out on -ve edge, in on +ve edge
    pub fn mpsse_transfer_tdi_bits_lsb_mode0(&self, byte: u8, nbits: u8) -> u8 {
        let div = self.mpsse_divisor.load(Ordering::SeqCst);

        extern "C" {
            fn _transfer_tdi_bits_lsb_mode0_4mhz(byte: u8, nbits: u8) -> u8;
            fn _transfer_tdi_bits_lsb_mode0_2p8mhz(byte: u8, nbits: u8) -> u8;
            fn _transfer_tdi_bits_lsb_mode0_2mhz(byte: u8, nbits: u8) -> u8;
            fn _transfer_tdi_bits_lsb_mode0_delay(byte: u8, nbits: u8, delay: u32) -> u8;
        }

        unsafe {
            match div {
                0 => 0x00,
                1 => _transfer_tdi_bits_lsb_mode0_4mhz(byte, nbits), // 6MHz -> 4MHz
                2 => _transfer_tdi_bits_lsb_mode0_2p8mhz(byte, nbits), // 3MHz -> 2.8MHz
                3 => _transfer_tdi_bits_lsb_mode0_2mhz(byte, nbits),
                n => _transfer_tdi_bits_lsb_mode0_delay(byte, nbits, n - 4),
            }
        }
    }

    // [0x4B] Clock Data to TMS pin (no read)
    // TMS with LSB first on -ve clk edge - use if clk is set to '0'
    // Bit 7 of the Byte1 is passed on to TDI/DO before the first clk of TMS
    // and is held static for the duration of TMS clocking.
    pub fn mpsse_write_tms_bits_lsb_mode0(&self, byte: u8, nbits: u8) {
        if nbits > 7 {
            return;
        }

        let div = self.mpsse_divisor.load(Ordering::SeqCst);

        extern "C" {
            fn _write_tms_bits_mode0_6mhz(byte: u8, nbits: u8);
            fn _write_tms_bits_mode0_3mhz(byte: u8, nbits: u8);
            fn _write_tms_bits_mode0_delay(byte: u8, nbits: u8, delay: u32);
        }

        unsafe {
            match div {
                0 => {},
                1 => _write_tms_bits_mode0_6mhz(byte, nbits),
                2 => _write_tms_bits_mode0_3mhz(byte, nbits),
                n => _write_tms_bits_mode0_delay(byte, nbits, n - 3),
            }
        }
    }

    // [0x6B] Clock Data to TMS pin with read
    // TMS with LSB first on -ve clk edge, read on +ve edge - use if clk is set to '0'
    // Bit 7 of the Byte1 is passed on to TDI/DO before the first clk of TMS
    // and is held static for the duration of TMS clocking. The TDO/DI pin is
    // sampled for the duration of TMS and a byte containing the data is passed
    // back at the end of TMS clocking.
    pub fn mpsse_transfer_tms_bits_lsb_mode0(&self, byte: u8, nbits: u8) -> u8 {
        if nbits > 7 {
            return 0x00;
        }

        let div = self.mpsse_divisor.load(Ordering::SeqCst);

        extern "C" {
            fn _transfer_tms_bits_mode0_4mhz(byte: u8, nbits: u8) -> u8;
            fn _transfer_tms_bits_mode0_2p8mhz(byte: u8, nbits: u8) -> u8;
            fn _transfer_tms_bits_mode0_2mhz(byte: u8, nbits: u8) -> u8;
            fn _transfer_tms_bits_mode0_delay(byte: u8, nbits: u8, delay: u32) -> u8;
        }

        unsafe {
            match div {
                0 => 0x00,
                1 => _transfer_tms_bits_mode0_4mhz(byte, nbits), // 6MHz -> 4MHz
                2 => _transfer_tms_bits_mode0_2p8mhz(byte, nbits), // 3MHz -> 2.8MHz
                3 => _transfer_tms_bits_mode0_2mhz(byte, nbits),
                n => _transfer_tms_bits_mode0_delay(byte, nbits, n - 4),
            }
        }
    }

    pub fn serial_set_baud_rate(&self, baud_rate: u32) {
        // TODO
    }
}
