// src/error.rs
use ufmt::derive::uDebug;

#[derive(Debug, uDebug)]
pub enum BuilderError {
    NoI2cConnected,
    InitFailed,
}

#[derive(Debug, uDebug)]
pub enum Sh1107gError<I2cE> {
    Builder(BuilderError),
    PayloadOverflow,
    I2cError(I2cE),
}

// BuilderErrorをSh1107gErrorに変換するためのFrom実装
// `Sh1107gBuilder::build`メソッドで`?`演算子を使用するために必要
impl From<BuilderError> for Sh1107gError<core::convert::Infallible> {
    fn from(e: BuilderError) -> Self {
        Sh1107gError::Builder(e)
    }
}

// PayloadOverflowをSh1107gErrorに変換するためのFrom実装

impl<I2cE> From<I2cE> for Sh1107gError<I2cE> {
    fn from(e: I2cE) -> Self {
        Sh1107gError::I2cError(e)
    }
}
