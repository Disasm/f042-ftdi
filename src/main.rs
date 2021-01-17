//! CDC-ACM serial port example using polling in a busy loop.
//! Target board: NUCLEO-F042K6
#![no_std]
#![no_main]

extern crate panic_rtt_target;

mod buffer;
mod ftdi;
mod hardware;

use crate::ftdi::{FtdiMode, FtdiPort};
use crate::hardware::Hardware;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f0xx_hal::rcc::HSEBypassMode;
use stm32f0xx_hal::usb::{Peripheral, UsbBus};
use stm32f0xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut dp = pac::Peripherals::take().unwrap();

    /*
     * IMPORTANT: if you have a chip in TSSOP20 (STM32F042F) or UFQFPN28 (STM32F042G) package,
     * and want to use USB, make sure you call `remap_pins(rcc, syscfg)`, otherwise the device will not enumerate.
     *
     * Uncomment the following function if the situation above applies to you.
     */
    #[cfg(feature = "tssop20")]
    stm32f0xx_hal::usb::remap_pins(&mut dp.RCC, &mut dp.SYSCFG);

    let rcc_cfgr = dp.RCC.configure();
    #[cfg(feature = "tssop20")]
    let rcc_cfgr = rcc_cfgr.hsi48().enable_crs(dp.CRS);
    #[cfg(not(feature = "tssop20"))]
    let rcc_cfgr = rcc_cfgr.hse(8.mhz(), HSEBypassMode::Bypassed);
    let mut rcc = rcc_cfgr.sysclk(48.mhz()).pclk(48.mhz()).freeze(&mut dp.FLASH);

    let gpioa = dp.GPIOA.split(&mut rcc);

    // Construct fake critical section
    let cs = unsafe { core::mem::zeroed() };
    let tms = gpioa.pa4.into_push_pull_output_hs(&cs);
    let tck = gpioa.pa5.into_push_pull_output_hs(&cs);
    let tdo = gpioa.pa6.into_floating_input(&cs);
    let tdi = gpioa.pa7.into_push_pull_output_hs(&cs);
    drop(cs);

    let hw = Hardware::new();

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
    }
}
