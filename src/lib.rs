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

pub const DISPLAY_WIDTH: u32 = 128;
pub const DISPLAY_HEIGHT: u32 = 128;
pub const BUFFER_SIZE: usize = DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize / 8;

pub struct Sh1107g<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
    pub(crate) buffer: [u8; BUFFER_SIZE],
}

impl<I2C, E> Sh1107g<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: embedded_hal::i2c::Error,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
    Self {
        i2c,
        address,
        buffer: [0u8; BUFFER_SIZE],
    }
}

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    pub fn clear_buffer(&mut self) {
        for b in self.buffer.iter_mut() {
            *b = 0;
        }
    }
}

pub struct Sh1107gBuilder<I2C> {
    i2c: Option<I2C>,
    address: u8,
    clear_on_init: bool,
}

impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: embedded_hal::i2c::Error,
{
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c: Some(i2c),
            address: 0x3C,
            clear_on_init: false,
        }
    }

    pub fn clear_on_init(mut self, enable: bool) -> Self {
        self.clear_on_init = enable;
        self
    }

    pub fn build(mut self) -> Sh1107g<I2C> {
        let i2c = self.i2c.take().expect("I2C must be set");
        let mut display = Sh1107g::new(i2c, self.address);
        if self.clear_on_init {
            display.clear_buffer();
        }
        display
    }
}


impl<I2C, E> Dimensions for Sh1107g<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: embedded_hal::i2c::Error,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT))
    }
}

impl<I2C, E> DrawTarget for Sh1107g<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: embedded_hal::i2c::Error,

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