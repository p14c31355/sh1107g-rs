use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};

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

pub struct Sh1107g<I2C> {
    i2c: I2C,
    address: u8,
    buffer: [u8; BUFFER_SIZE], // Internal buffer
    // Configure in builder to Sh1107g struct
}


// Sh1107g に Dimensions トレイトを実装
impl<I2C> Dimensions for Sh1107g<I2C> {
    fn bounding_box(&self) -> embedded_graphics_core::primitives::Rectangle {
        embedded_graphics_core::primitives::Rectangle::new(
            Point::new(0, 0),
            Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT),
        )
    }
}

impl<I2C, E> DrawTarget for Sh1107g<I2C>
where
    // I2Cトレイト境界は、DrawTarget自身はI2cトレイトを必要としないため、ここで指定する必要はない
    // むしろ、Sh1107gがI2CとEに依存していることを示すだけでよい
    Sh1107g<I2C>: Sized, // Self::Error が E であることを保証するため
    E: embedded_hal_async::i2c::Error + embedded_hal::i2c::Error, // 両方のエラー型に対応
{
    // DrawTarget define color dimension (monochro OLED = BinaryColor)
    type Color = BinaryColor;
    type Error = E; // embedded-halのI2Cエラーをそのまま使う

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