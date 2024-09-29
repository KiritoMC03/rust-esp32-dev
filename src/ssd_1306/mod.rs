use anyhow::{Error, Result};
use embedded_hal::spi::{Mode, Phase, Polarity};
use esp_idf_hal::gpio::{self, Gpio5, Gpio16, Gpio18, Gpio23, Output, OutputPin, PinDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::prelude::FromValueType;
use esp_idf_hal::spi::{self, SpiAnyPins, SpiDeviceDriver, SpiDriver, SpiDriverConfig};
use esp_idf_hal::spi::config::Config;
use ssd1306::prelude::*;

#[allow(dead_code)]
pub fn spi_interface_default<'s>(
    spi: spi::SPI2,
    clock: Gpio18,
    mosi_data: Gpio23,
    cs: Gpio5,
    dc: Gpio16,
) -> Result<SPIInterface<SpiDeviceDriver<'s, SpiDriver<'s>>, PinDriver<'s, Gpio16, Output>>, Error>
{
    spi_interface(
        spi,
        clock,
        mosi_data,
        cs,
        PinDriver::output(dc)?,
    )
}

#[allow(dead_code)]
pub fn spi_interface<'s, DC>(
    spi: impl Peripheral<P = impl SpiAnyPins> + 's,
    clock: impl Peripheral<P = impl OutputPin> + 's,
    mosi_data: impl Peripheral<P = impl OutputPin> + 's,
    cs: impl Peripheral<P = impl OutputPin> + 's,
    dc: DC,
) -> Result<SPIInterface<SpiDeviceDriver<'s, SpiDriver<'s>>, DC>, Error>
    where DC: embedded_hal::digital::OutputPin
{
    let spi_driver = SpiDriver::new(
        spi,
        clock,
        mosi_data,
        None::<gpio::AnyIOPin>,
        &SpiDriverConfig::new(),
    )?;
    let config = Config::new().baudrate(1.MHz().into()).data_mode(Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    });
    let spi_device_driver = SpiDeviceDriver::new(spi_driver, Some(cs), &config)?;
    let interface = SPIInterface::new(spi_device_driver, dc);
    Ok(interface)
}