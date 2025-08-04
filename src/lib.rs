#![no_std]
/// SH1107G I2C OLED driver
pub mod cmds;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async_")]
pub mod async_;

use embedded_graphics_core::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    Pixel,
};
use core::convert::Infallible;
use core::result::Result;
use core::option::Option::{self, Some, None};
use core::result::Result::Ok;
use core::fmt::Debug;
use core::iter::IntoIterator;

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
use embedded_graphics_core::geometry::{Dimensions, Point, Size};

pub struct Sh1107g<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
    pub(crate) buffer: [u8; BUFFER_SIZE], // Internal buffer
    // Configure in builder to Sh1107g struct
}

impl <I2C> Sh1107g<I2C> {
    // Make new driver instance & Degine function called by the builder
    // Initialise the internal buffer when called by builder
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
            buffer: [0x00; BUFFER_SIZE], // 全てオフで初期化
        }
    }

    /// 内部バッファをクリアする
    pub fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|b| *b = 0x00);
    }
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
    InitFailed,
    // NoDisplaySizeDefined, // サイズが必須の場合
}

#[derive(Debug)]
pub enum Sh1107gError<I2cE> {
    Builder(BuilderError),
    PayloadOverflow,
    I2cError(I2cE),
}

// embedded-halのErrorトレイトにも対応させる必要があるかもしれません
// impl embedded_hal::i2c::Error for BuilderError { ... }
// impl From<BuilderError> for YourDriverError { ... } など

// From 実装で ? を使えるように
impl<I2cE> From<I2cE> for Sh1107gError<I2cE> {
    fn from(e: I2cE) -> Self {
        Sh1107gError::I2cError(e)
    }
}

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

impl<I2C> DrawTarget for Sh1107g<I2C> {
    // DrawTarget define color dimension (monochro OLED = BinaryColor)
    type Color = BinaryColor;
    type Error = Infallible; // embedded-halのI2Cエラーをそのまま使う
    
    /// ピクセルを描画する主要なメソッド
    fn draw_iter<PIXELS>(&mut self, pixels: PIXELS) -> Result<(), Self::Error>
    where
        PIXELS: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(Point { x, y }, color) in pixels {
            // 座標がディスプレイ範囲内かチェック
            if x < 0 || x >= DISPLAY_WIDTH as i32 || y < 0 || y >= DISPLAY_HEIGHT as i32 {
                continue; // 範囲外のピクセルはスキップ
            }

            // ピクセル座標からバッファのインデックスとビットマスクを計算
            // SH1107Gはページアドレッシングモードで、各バイトが縦8ピクセル
            let byte_index = (x as usize) + (y as usize / 8) * (DISPLAY_WIDTH as usize);
            let bit_mask = 1 << (y % 8); // バイト内のビット位置

            // バッファの範囲チェック（念のため）
            if byte_index >= BUFFER_SIZE {
                continue; // バッファ範囲外もスキップ
            }

            // 色に応じてバッファのビットをセットまたはクリア
            match color {
                BinaryColor::On => self.buffer[byte_index] |= bit_mask,  // ピクセルをON (セット)
                BinaryColor::Off => self.buffer[byte_index] &= !bit_mask, // ピクセルをOFF (クリア)
            }
        }
        Ok(())
    }

    /// Fill in with color
    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let fill_byte = match color {
            BinaryColor::On => 0xFF,
            BinaryColor::Off => 0x00,
        };
        self.buffer.iter_mut().for_each(|b| *b = fill_byte);
        Ok(())
    }
}
