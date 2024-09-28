use anyhow::Result;
use esp_idf_hal::gpio::{InputPin, OutputPin, Pin, PinDriver};
use esp_idf_hal::gpio::{Input, InterruptType, Pull};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_sys::EspError;
use crate::helpers;

#[allow(dead_code)]
pub struct ICLineChecker<'a, T: Pin> {
    pin: PinDriver<'a, T, Input>,
    pub true_if_low: bool,
}

impl<'a, T: InputPin + OutputPin> ICLineChecker<'a, T> {
    pub fn from_pin(pin: impl Peripheral<P = T> + 'a) -> Result<Self, EspError> {
        Ok(Self {
            pin: helpers::pin::input_pin_pull_intrp(pin, Pull::Up, InterruptType::AnyEdge)?,
            true_if_low: true,
        })
    }

    pub fn check(&mut self) -> bool {
        return if self.pin.is_low() {
            self.true_if_low
        } else {
            !self.true_if_low
        }        
    }
}