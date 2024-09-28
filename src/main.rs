mod lcd;
mod blink;
mod uuid;
mod rgb_led;
mod wifi;
mod hc_sr04;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::timer::{TimerConfig, TimerDriver};
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    prepare();
    let peripherals = Peripherals::take().unwrap();
    let trigger = hc_sr04::Trigger::from_pin(peripherals.pins.gpio21)?;
    let echo = hc_sr04::Echo::from_pin(peripherals.pins.gpio19)?;
    let mut hc_sr04 = hc_sr04::HCSR04::new(trigger, echo);

    loop {
        log::info!("Distance: {:.2} cm", hc_sr04.measure_avg(10));
        FreeRtos::delay_ms(100);
    }
}

fn prepare() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello, world!");
    esp_idf_hal::sys::link_patches();
    log::info!("Ready");
}