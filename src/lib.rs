/// SH1107G I2C OLED driver
pub mod cmds;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod async_;

use heapless::Vec;

use crate::cmds::*;

/*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayRotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

// Default process
impl Default for DisplayRotation {
    fn default() -> Self {
        DisplayRotation::Rotate0
    }
}
*/

/// common
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

pub struct Sh1107g<I2C> {
    i2c: I2C,
    address: u8,
    buffer: [u8; BUFFER_SIZE], // Internal buffer
    // Configure in builder to Sh1107g struct
}

// Builder struct
pub struct Sh1107gBuilder<I2C> {
    i2c: Option<I2C>,
    address: u8,      // Configure default address or choice Option type
    // If you can add more settings value rotation: DisplayRotation,etc...
}

// Sh1107gBuilder implement block
impl<I2C> Sh1107gBuilder<I2C> {
    /// Make new builder instance
    /// Designation default I2C address
    pub fn new() -> Self {
        Self {
            i2c: None,
            address: 0x3C, // default
            // size: None,
            // rotation: DisplayRotation::default(),
        }
    }

    /// Connect I2C
    pub fn connect_i2c(mut self, i2c: I2C) -> Self {
        self.i2c = Some(i2c);
        self
    }

    /// Configure I2C address
    pub fn with_address(mut self, address: u8) -> Self {
        self.address = address;
        self
    }

    // If you need other method, add other setting method, example: size,rotate,etc...
}

// Define error enum in builder
#[derive(Debug)]
pub enum BuilderError {
    NoI2cConnected,
    // NoDisplaySizeDefined, // サイズが必須の場合
}
// embedded-halのErrorトレイトにも対応させる必要があるかもしれません
// impl embedded_hal::i2c::Error for BuilderError { ... }
// impl From<BuilderError> for YourDriverError { ... } など

// define display size
pub const DISPLAY_WIDTH: u32 = 128;
pub const DISPLAY_HEIGHT: u32 = 128;
pub const BUFFER_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT / 8) as usize;

// Sh1107g に Dimensions トレイトを実装
impl<I2C> Dimensions for Sh1107g<I2C> {
    fn bounding_box(&self) -> embedded_graphics_core::primitives::Rectangle {
        embedded_graphics_core::primitives::Rectangle::new(
            Point::new(0, 0),
            Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT),
        )
    }
}
