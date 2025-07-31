// Builder struct
pub struct Sh1107gBuilder<I2C> {
    i2c: Option<I2C>,
    address: u8,      // Configure default address or choice Option type
    // If you can add more settings value rotation: DisplayRotation,etc...
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
const DISPLAY_WIDTH: u32 = 128;
const DISPLAY_HEIGHT: u32 = 128;
const BUFFER_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT / 8) as usize;

pub struct Sh1107g<I2C> {
    i2c: I2C,
    address: u8,
    buffer: [u8; BUFFER_SIZE], // Internal buffer
    // Configure in builder to Sh1107g struct
}