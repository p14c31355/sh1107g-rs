#![no_std]

pub mod cmds;
pub mod error;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async_")]
pub mod async_;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    primitives::Rectangle,
    Pixel,
};

use core::{
    convert::Infallible,
    result::Result,
    option::Option::{self, Some},
};

#[cfg(feature = "debug_log")]
use dvcdbg::logger::{Logger, SerialLogger};
#[cfg(not(feature = "debug_log"))]
use dvcdbg::logger::NoopLogger;

#[cfg(feature = "debug_log")]
pub type DefaultLogger<'a, W> = SerialLogger<'a, W>;
#[cfg(not(feature = "debug_log"))]
pub type DefaultLogger = NoopLogger;

pub const DISPLAY_WIDTH: u32 = 128;
pub const DISPLAY_HEIGHT: u32 = 128;
pub const BUFFER_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT / 8) as usize;

// LはOptionでラップされているため、`?Sized`は不要です。
// `Option`はジェネリック型パラメータを持つため、ライフタイム `'a` が必要になります。
pub struct Sh1107g<'a, I2C, L> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
    pub(crate) buffer: [u8; BUFFER_SIZE],
    pub(crate) logger: Option<&'a mut L>,
}

impl<'a, I2C, L> Sh1107g<'a, I2C, L>
where
    L: Logger,
{
    pub fn new(i2c: I2C, address: u8, logger: Option<&'a mut L>) -> Self {
        Self {
            i2c,
            address,
            buffer: [0x00; BUFFER_SIZE],
            logger,
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|b| *b = 0x00);
    }

    pub fn with_logger<F: FnOnce(&mut L)>(&mut self, f: F) {
        if let Some(logger) = self.logger.as_mut() {
            f(logger);
        }
    }
}

// L は Logger を実装する必要があるため、`where`句に追加します。
pub struct Sh1107gBuilder<'a, I2C, L>
where
    L: Logger,
{
    i2c: Option<I2C>,
    address: u8,
    logger: Option<&'a mut L>,
}

impl<'a, I2C, L> Sh1107gBuilder<'a, I2C, L>
where
    L: Logger,
{
    pub fn new(i2c: I2C, logger: &'a mut L) -> Self {
        Self {
            i2c: Some(i2c),
            address: 0x3C,
            logger: Some(logger),
        }
    }
    
    pub fn with_address(mut self, address: u8) -> Self {
        self.address = address;
        self
    }

    pub fn build(self) -> Sh1107g<'a, I2C, L> {
        Sh1107g::new(
            self.i2c.expect("I2C must be set"),
            self.address,
            self.logger,
        )
    }
}

impl<'a, I2C, L> Dimensions for Sh1107g<'a, I2C, L>
where
    L: Logger,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT))
    }
}

impl<'a, I2C, L> DrawTarget for Sh1107g<'a, I2C, L>
where
    L: Logger,
{
    type Color = BinaryColor;
    type Error = Infallible;

    fn draw_iter<PIXELS>(&mut self, pixels: PIXELS) -> Result<(), Self::Error>
    where
        PIXELS: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(Point { x, y }, color) in pixels {
            if x < 0 || x >= DISPLAY_WIDTH as i32 || y < 0 || y >= DISPLAY_HEIGHT as i32 {
                continue;
            }

            let byte_index = (x as usize) + (y as usize / 8) * (DISPLAY_WIDTH as usize);
            let bit_mask = 1 << (y % 8);

            if byte_index >= BUFFER_SIZE {
                continue;
            }

            match color {
                BinaryColor::On => self.buffer[byte_index] |= bit_mask,
                BinaryColor::Off => self.buffer[byte_index] &= !bit_mask,
            }
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let fill_byte = match color {
            BinaryColor::On => 0xFF,
            BinaryColor::Off => 0x00,
        };
        self.buffer.iter_mut().for_each(|b| *b = fill_byte);
        Ok(())
    }
}