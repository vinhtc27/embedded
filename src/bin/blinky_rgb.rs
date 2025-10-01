#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal_smartled::{smart_led_buffer, SmartLedsAdapter};
use smart_leds::{SmartLedsWrite as _, RGB8};
use {esp_backtrace as _, esp_println as _};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, smart_led_buffer!(1));

    const LEVEL: u8 = 10;

    led.write([RGB8::new(0, 0, 0)].into_iter()).unwrap();
    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(5000) {}

    let mut color = RGB8::new(LEVEL, 0, 0);
    loop {
        info!("Blink!");
        led.write([color].into_iter()).unwrap();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
        let tmp = color.r;
        color.r = color.g;
        color.g = color.b;
        color.b = tmp;
    }
}
