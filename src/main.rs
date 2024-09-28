mod lcd;
mod blink;
mod uuid;
mod rgb_led;
mod wifi;
mod ic_line_check;
mod hc_sr04;
mod helpers;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;
use crate::ic_line_check::ICLineChecker;

fn main() -> anyhow::Result<()> {
    prepare();
    let peripherals = Peripherals::take().unwrap();
    let ic = peripherals.pins.gpio21;
    let mut checker = ICLineChecker::from_pin(ic)?;
    loop {
        log::info!("IC Line is {}", checker.check());
        FreeRtos::delay_ms(1000);
    }
    Ok(())
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