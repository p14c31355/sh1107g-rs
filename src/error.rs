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