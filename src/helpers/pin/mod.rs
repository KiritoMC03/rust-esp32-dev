use anyhow::Result;
use esp_idf_hal::gpio::{Input, InputPin, InterruptType, OutputPin, PinDriver, Pull};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_sys::EspError;

#[allow(dead_code)]
pub fn input_pin_pull<'a, T: InputPin + OutputPin>(
    pin: impl Peripheral<P = T> + 'a,
    pull: Pull
) -> Result<PinDriver<'a, T, Input>, EspError>
{
    let mut pin = PinDriver::input(pin)?;
    pin.set_pull(pull)?;
    Ok(pin)
}

#[allow(dead_code)]
pub fn input_pin_pull_intrp<'a, T: InputPin + OutputPin>(
    pin: impl Peripheral<P = T> + 'a,
    pull: Pull,
    interrupt_type: InterruptType
) -> Result<PinDriver<'a, T, Input>, EspError> 
{
    let mut pin = PinDriver::input(pin)?;
    pin.set_pull(pull)?;
    pin.set_interrupt_type(interrupt_type)?;
    pin.enable_interrupt()?;
    Ok(pin)
}