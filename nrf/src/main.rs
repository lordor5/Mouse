#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};
use defmt::info;

#![embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let mut led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);

    info!("Starting blink example");

    loop {
        led.set_high();
        Timer::after(core::time::Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(core::time::Duration::from_millis(500)).await;
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}