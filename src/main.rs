//! CDC-ACM serial port example using polling in a busy loop.
//! Target board: NUCLEO-F042K6
#![no_std]
#![no_main]

extern crate panic_rtt_target;

mod buffer;
mod ftdi;
mod hardware;

use crate::ftdi::{FtdiPort, FtdiMode};
use crate::hardware::Hardware;
use cortex_m_rt::entry;
use stm32f0xx_hal::usb::{Peripheral, UsbBus};
use stm32f0xx_hal::{pac, prelude::*};
use stm32f0xx_hal::rcc::HSEBypassMode;
use rtt_target::{rprintln, rtt_init_print};

#[inline(never)]
#[no_mangle]
unsafe extern "C" fn jtag_write(mut data_ptr: *const u8, data_size: usize, bsrr: *mut u32, tck_tdi: u32) {
    let tck0_tdi1 = tck_tdi;
    let tck1 = tck0_tdi1 >> 16;
    let tdi0 = tck0_tdi1 << 16;
    let tdi1 = tdi0 >> 16;
    let tck0_tdi0 = (tdi0 | tck0_tdi1) ^ tdi1;

    for _ in 0..data_size {
        let byte = data_ptr.read_volatile();
        data_ptr = data_ptr.add(1);

        for i in 0..8 {
            if (byte >> i) & 1 != 0 {
                bsrr.write_volatile(tck0_tdi1);
            } else {
                bsrr.write_volatile(tck0_tdi0);
            }
            bsrr.write_volatile(tck1);
        }
    }
}

#[inline(never)]
#[no_mangle]
unsafe extern "C" fn jtag_write2(mut data_ptr: *const u8, data_size: usize, bsrr: *mut u32, tck_tdi: u32) {
    let tck0_tdi1 = tck_tdi;
    let tck1 = tck0_tdi1 >> 16;
    let tdi0 = tck0_tdi1 << 16;
    let tck0_tdi0_tdi1 = tdi0 | tck0_tdi1;

    for _ in 0..data_size {
        let byte = !data_ptr.read_volatile();
        data_ptr = data_ptr.add(1);

        for i in 0..8 {
            bsrr.write_volatile(tck0_tdi0_tdi1 & !((byte as u32) << (7 - i)));
            bsrr.write_volatile(tck1);
        }
    }
}

fn test() {
    const SCK1: u32 = (1u32 << 5);
    const SCK0: u32 = SCK1 << 16;
    const TDI1: u32 = (1u32 << 7);
    const TDI0: u32 = TDI1 << 16;
    const SS1: u32 = (1u32 << 4);
    const SS0: u32 = SS1 << 16;

    let gpio = unsafe { &*pac::GPIOA::ptr() };
    let bsrr = &gpio.bsrr as *const _ as *mut u32;

    unsafe {
        bsrr.write_volatile(SCK0 | TDI1);
        cortex_m::asm::delay(10);
        bsrr.write_volatile(SCK1);
        cortex_m::asm::delay(10);

        bsrr.write_volatile(SCK0 | TDI0);
        cortex_m::asm::delay(10);
        bsrr.write_volatile(SCK1);
        cortex_m::asm::delay(10);

        bsrr.write_volatile(SCK0 | TDI0 | TDI1);
        cortex_m::asm::delay(10);
        bsrr.write_volatile(SCK1);
        cortex_m::asm::delay(10);

        bsrr.write_volatile(SCK0);
        cortex_m::asm::delay(10);
        bsrr.write_volatile(SCK1);
        cortex_m::asm::delay(10);
    }

    let buf = [0x5A; 16];
    // unsafe { jtag_write(buf.as_ptr(), buf.len(), bsrr, SCK0 | TDI1); }
    // cortex_m::asm::delay(50);
    // unsafe { jtag_write2(buf.as_ptr(), buf.len(), bsrr, SCK0 | TDI1); }
    // cortex_m::asm::delay(50);

    unsafe {
        bsrr.write_volatile(SCK0);
        cortex_m::asm::delay(20);

        extern "C" {
            fn _write_tdi_bytes_lsb_mode0_6mhz(data_ptr: *const u8, data_size: usize, bsrr: *mut u32);
            fn _write_tdi_bytes_lsb_mode0_3mhz(data_ptr: *const u8, data_size: usize, bsrr: *mut u32);
            fn _write_tdi_bytes_lsb_mode0_delay(data_ptr: *const u8, data_size: usize, bsrr: *mut u32, delay: u32);
            fn _write_tdi_bits_lsb_mode0_6mhz(byte: u8, nbits: u8, bsrr: *mut u32);
            fn _write_tdi_bits_lsb_mode0_3mhz(byte: u8, nbits: u8, bsrr: *mut u32);
            fn _write_tdi_bits_lsb_mode0_delay(byte: u8, nbits: u8, bsrr: *mut u32, delay: u32);
        }
        bsrr.write_volatile(SS0);

        _write_tdi_bits_lsb_mode0_6mhz(0b10110010, 8, bsrr);
        cortex_m::asm::delay(10);

        _write_tdi_bits_lsb_mode0_3mhz(0b10110010, 8, bsrr);
        cortex_m::asm::delay(10);

        _write_tdi_bits_lsb_mode0_delay(0b10110010, 8, bsrr, 0);
        cortex_m::asm::delay(10);

        _write_tdi_bits_lsb_mode0_delay(0b10110010, 8, bsrr, 1);
        cortex_m::asm::delay(10);

        _write_tdi_bits_lsb_mode0_delay(0b10110010, 8, bsrr, 2);
        cortex_m::asm::delay(10);

        // _write_tdi_bytes_lsb_mode0_6mhz(buf.as_ptr(), buf.len(), bsrr);
        // cortex_m::asm::delay(20);
        //
        // _write_tdi_bytes_lsb_mode0_3mhz(buf.as_ptr(), buf.len(), bsrr);
        // cortex_m::asm::delay(20);
        //
        // _write_tdi_bytes_lsb_mode0_delay(buf.as_ptr(), buf.len(), bsrr, 0);

        bsrr.write_volatile(SS1);
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut dp = pac::Peripherals::take().unwrap();

    //let mut buf = [0x22u8; 64];
    //unsafe { jtag_write(buf.as_ptr(), buf.len(), 0xdead0000 as usize as _, 0x1000_0100); }

    /*
     * IMPORTANT: if you have a chip in TSSOP20 (STM32F042F) or UFQFPN28 (STM32F042G) package,
     * and want to use USB, make sure you call `remap_pins(rcc, syscfg)`, otherwise the device will not enumerate.
     *
     * Uncomment the following function if the situation above applies to you.
     */

    //stm32f0xx_hal::usb::remap_pins(&mut dp.RCC, &mut dp.SYSCFG);

    let mut rcc = dp
        .RCC
        .configure()
        //.hsi48()
        .hse(8.mhz(), HSEBypassMode::Bypassed)
        .enable_crs(dp.CRS)
        .sysclk(48.mhz())
        .pclk(48.mhz())
        .freeze(&mut dp.FLASH);

    cortex_m::asm::delay(1000);

    // Configure the on-board LED (LD3, green)
    // let gpiob = dp.GPIOB.split(&mut rcc);
    // let mut led = cortex_m::interrupt::free(|cs| gpiob.pb3.into_push_pull_output(cs));
    // led.set_low(); // Turn off

    let gpioa = dp.GPIOA.split(&mut rcc);

    // Construct fake critical section
    let cs = unsafe { core::mem::zeroed() };
    let tms = gpioa.pa4.into_push_pull_output_hs(&cs);
    let tck = gpioa.pa5.into_push_pull_output_hs(&cs);
    let tdo = gpioa.pa6.into_floating_input(&cs);
    let tdi = gpioa.pa7.into_push_pull_output_hs(&cs);
    drop(cs);

    test();

    let hw = Hardware;

    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
    };

    let usb_bus = UsbBus::new(usb);
    let mut port_a = FtdiPort::new(&usb_bus, &hw, FtdiMode::MPSSE);
    let mut port_b = FtdiPort::new(&usb_bus, &hw, FtdiMode::Serial);
    let mut usb_dev = FtdiPort::make_device(&usb_bus);

    loop {
        usb_dev.poll(&mut [&mut port_a, &mut port_b]);
        port_a.process_commands();
        port_b.process_commands();

        //test();
        cortex_m::asm::delay(100);
    }
}
