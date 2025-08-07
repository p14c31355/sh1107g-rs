#![no_std]
/// SH1107G I2C OLED driver
pub mod cmds;
pub mod error;

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

#[cfg(feature = "debug_log")]
use dvcdbg::logger::{Logger, SerialLogger};
#[cfg(not(feature = "debug_log"))]
use dvcdbg::logger::NoopLogger;

#[cfg(feature = "debug_log")]
pub type DefaultLogger<'a, W> = SerialLogger<'a, W>;
#[cfg(not(feature = "debug_log"))]
pub type DefaultLogger = NoopLogger;

pub struct Sh1107g<'a, I2C, L = DefaultLogger<'a, W>> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
    pub(crate) buffer: [u8; BUFFER_SIZE], // Internal buffer
    pub(crate) logger: L,
    // Configure in builder to Sh1107g struct
}

impl <I2C, L> Sh1107g<I2C, L>{
    // Make new driver instance & Degine function called by the builder
    // Initialise the internal buffer when called by builder
    pub fn new(i2c: I2C, address: u8, logger: L) -> Self {
        Self {
            i2c,
            address,
            buffer: [0x00; BUFFER_SIZE], // 全てオフで初期化
            logger,
        }
    }

    /// 内部バッファをクリアする
    pub fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|b| *b = 0x00);
    }
    
    #[cfg(feature = "debug_log")]
    pub fn new_with_logger(i2c: I2C, address: u8, logger: L) -> Self {
        Self {
            i2c,
            address,
            buffer: [0; BUFFER_SIZE],
            logger,
        }
    }

}
// Builder struct
pub struct Sh1107gBuilder<'a, I2C, L: Logger = DefaultLogger<'a, W>> {
    i2c: Option<I2C>,
    address: u8,      // Configure default address or choice Option type
    logger: Option<L>,
    // If you can add more settings value rotation: DisplayRotation,etc...
}

// Sh1107gBuilder implement block
impl<I2C, L: Logger + core::default::Default> Sh1107gBuilder<I2C, L> {
    /// Make new builder instance
    /// Designation default I2C address
    pub fn new() -> Self {
        Self {
            i2c: None,
            address: 0x3C, // default
            // size: None,
            // rotation: DisplayRotation::default(),
            logger: None,
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

    pub fn with_logger(mut self, logger: L) -> Self {
        self.logger = Some(logger);
        self
    }

    pub fn build_log(self) -> Sh1107g<I2C, L> {
        Sh1107g::new(
            self.i2c.expect("I2C must be set"),
            self.address,
            self.logger.unwrap_or_else(L::default),
        )
    }
    // If you need other method, add other setting method, example: size,rotate,etc...
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
