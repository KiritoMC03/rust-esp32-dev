use anyhow::{Error, Result};
use display_interface::DisplayError;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Line, PrimitiveStyle};
use embedded_hal::spi::{Mode, Phase, Polarity};
use esp_idf_hal::gpio::{self, Gpio16, Gpio18, Gpio23, Gpio5, Output, OutputPin, PinDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::prelude::FromValueType;
use esp_idf_hal::spi::{self, SpiAnyPins, SpiDeviceDriver, SpiDriver, SpiDriverConfig};
use esp_idf_hal::spi::config::Config;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::prelude::*;
use ssd1306::Ssd1306;

use crate::helpers::graphics::*;
use crate::helpers::graphics::lines::*;

pub mod tests;

// https://energiazero.org/cartelle/arduino//arduino_applicazioni/esp32/electronicshub.org-how%20to%20interface%20oled%20display%20with%20esp32%20esp32%20oled%20display%20tutorial.pdf

#[allow(dead_code)]
pub fn spi_interface_default<'s>(
    spi: spi::SPI2,
    clock: Gpio18, // D0
    mosi_data: Gpio23, // D1 / SDA
    dc: Gpio16,
    cs: Gpio5,
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

impl<DI, SIZE> Flush for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize
{
    fn flush(&mut self) -> Result<(), DisplayError> {
        self.flush()?;
        Ok(())
    }
}

impl<DI, SIZE, C> DrawLine<C> for Ssd1306<DI, SIZE, BufferedGraphicsMode<SIZE>>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
    C: PixelColor + Default,
    BinaryColor: From<C>
{
    fn draw_single_line(&mut self, line: &lines::Line<C>) -> Result<(), DisplayError> {
        Line::new(line.point1(), line.point2())
            .into_styled(PrimitiveStyle::with_stroke(line.color.into(), line.stroke))
            .draw(self)
            .expect("Failed to draw line");
        self.flush()
    }
}