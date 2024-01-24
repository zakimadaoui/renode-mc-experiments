//! Serial interface write formatted strings test
//!
//! You need to connect the Tx pin to the Rx pin of a serial-usb converter
//! so you can see the message in a serial console (e.g. Arduino console).

#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m::asm;
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};

use core::fmt::Write;

pub mod shared {
    #[link_section = ".shared_data"]
    #[no_mangle]
    pub static mut MY_SHARED1: usize = 777;
}

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let pac = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = pac.FLASH.constrain();
    let rcc = pac.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = pac.AFIO.constrain();

    // Prepare the GPIOB peripheral
    // let mut gpioa = pac.GPIOA.split();
    let mut gpiob = pac.GPIOB.split();

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    // Configure pb10 as a push_pull output, this will be the tx pin
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // Take ownership over pb11
    let rx = gpiob.pb11;

    // Set up the usart device. Take ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.
    let serial = Serial::new(
        pac.USART3,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        &clocks,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, _rx) = serial.split();

    loop {
        unsafe {
            let val = core::ptr::read_volatile(&shared::MY_SHARED1);
            core::ptr::write_volatile(&mut shared::MY_SHARED1, val + 1);
            writeln!(tx, "Hello from core 2 {}", val).unwrap();
        }
        asm::delay(37_000_000);
    }
}
