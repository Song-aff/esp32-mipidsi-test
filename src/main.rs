#![no_std]
#![no_main]

/* --- Needed by ESP32-c3 --- */
use esp_backtrace as _;
use hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    gpio::{NO_PIN,IO},
    Delay
};
/* -------------------------- */

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Primitive, PrimitiveStyle, Rectangle, Triangle},
};

// Provides the parallel port and display interface builders
use display_interface_spi::SPIInterface;

// Provides the Display builder
use mipidsi::Builder;

use fugit::RateExtU32;
use esp_println::println;


#[entry]
fn main() -> ! {

    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    


    let dc = io.pins.gpio7.into_push_pull_output();
    let mut rst = io.pins.gpio8.into_push_pull_output();
    rst.set_high().unwrap();

    // Define the SPI pins and create the SPI interface
    let sck = io.pins.gpio5;
    // let miso = io.pins.gpio4;
    let mosi = io.pins.gpio6;
    let cs = io.pins.gpio10.into_push_pull_output();

    let spi = Spi::new(peripherals.SPI2, 60u32.MHz(), SpiMode::Mode0, &clocks).with_pins(
        Some(sck),
        Some(mosi),
        NO_PIN,
        NO_PIN,
    );

    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, delay);
    let di = SPIInterface::new(spi_device, dc);

    let display_modal = mipidsi::models::ST7735s;

    #[cfg(feature = "ST7735s")]
    let display_modal = mipidsi::models::ST7735s;
    #[cfg(feature = "ST7789")]
    let display_modal = mipidsi::models::ST7789;
    #[cfg(feature = "ST7796")]
    let display_modal = mipidsi::models::ST7796;
    #[cfg(feature = "ILI9341")]
    let display_modal = mipidsi::models::ILI9341Rgb565;
    #[cfg(feature = "ILI9342")]
    let display_modal = mipidsi::models::ILI9342CRgb565;
    #[cfg(feature = "ILI9386")]
    let display_modal = mipidsi::models::ILI9486Rgb565;

    let mut display = mipidsi::Builder::new(display_modal, di)
    // .display_size(160, 200)
    .reset_pin(rst)
    .init(&mut delay)
    .unwrap();

    
    // Make the display all black
    display.clear(Rgb565::BLACK).unwrap();
    // Draw a smiley face with white eyes and a red mouth
    // draw_smiley(&mut display).unwrap();
    demo(&mut display).unwrap();
    loop {
        // Do nothing
    }
}

fn draw_smiley<T: DrawTarget<Color = Rgb565>>(display: &mut T) -> Result<(), T::Error> {
    // Draw the left eye as a circle located at (50, 100), with a diameter of 40, filled with white
    Circle::new(Point::new(50, 100), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    // Draw the right eye as a circle located at (50, 200), with a diameter of 40, filled with white
    Circle::new(Point::new(50, 200), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;

    // Draw an upside down red triangle to represent a smiling mouth
    Triangle::new(
        Point::new(130, 140),
        Point::new(130, 200),
        Point::new(160, 170),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
    .draw(display)?;

    // Cover the top part of the mouth with a black triangle so it looks closed instead of open
    Triangle::new(
        Point::new(130, 150),
        Point::new(130, 190),
        Point::new(150, 170),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
    .draw(display)?;
    Ok(())
}

fn demo<T: DrawTarget<Color = Rgb565>>(display: &mut T) -> Result<(), T::Error> {
        // Draw the left eye as a circle located at (50, 100), with a diameter of 40, filled with white
        Circle::new(Point::new(0, 0), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::WHITE))
        .draw(display)?;
    Ok(())
}