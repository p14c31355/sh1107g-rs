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
use embedded_hal::i2c::I2c;

use core::{
    convert::Infallible,
    result::Result,
    option::Option::{self, Some},
};

use error::*;

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
use crate::cmds::*;

pub struct Sh1107g<I2C> {
    i2c: I2C,
    buffer: [u8; BUFFER_SIZE],
}

impl<I2C, E> Sh1107g<I2C>
where
    I2C: I2c<Error = E>,
    E: embedded_hal::i2c::Error,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c, buffer }
    }

    fn write_i2c(&mut self, control: u8, payload: &[u8]) -> Result<(), Sh1107gError<E>> {
        if payload.len() > 128 {
            return Err(Sh1107gError::PayloadOverflow);
        }
        let mut buf = [0u8; 129]; // control + up to 128 bytes
        buf[0] = control;
        buf[1..1 + payload.len()].copy_from_slice(payload);
        self.i2c.write(0x3C, &buf[..1 + payload.len()])?;
        Ok(())
    }

    pub fn send_cmd(&mut self, cmd: &[u8]) -> Result<(), Sh1107gError<E>> {
        self.write_i2c(0x00, cmd)
    }

    pub fn send_data(&mut self, data: &[u8]) -> Result<(), Sh1107gError<E>> {
        self.write_i2c(0x40, data)
    }

    pub fn init(&mut self) -> Result<(), Sh1107gError<E>> {
        self.send_cmd(SH1107G_INIT_CMDS)
    }

    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
        for page in 0..8 {
            self.send_cmd(&SetPageAddress(page).to_bytes())?;
            self.send_cmd(&SetColumnAddress(0).to_bytes())?;
            let start = page * 128;
            self.send_data(&self.buffer[start..start + 128])?;
        }
        Ok(())
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
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
    clear_on_init: bool,
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
            clear_on_init: false,
        }
    }

    pub fn clear_on_init(mut self, enable: bool) -> Self {
        self.clear_on_init = enable;
        self
    }

    pub fn build(mut self) -> Sh1107g<'a, I2C, L> {
        let mut display = Sh1107g::new(
            self.i2c.take().expect("I2C must be set"),
            self.address,
            self.logger,
        );
        if self.clear_on_init {
            display.clear_buffer();
        }
        display
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