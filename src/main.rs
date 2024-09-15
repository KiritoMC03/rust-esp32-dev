mod lcd;
mod blink;

use esp_idf_svc::hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    prepare();
    let _peripherals = Peripherals::take().unwrap();
    
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