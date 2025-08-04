// src/error.rs
use ufmt_macros::uDebug;

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
impl<I2cE> From<BuilderError> for Sh1107gError<I2cE> {
    fn from(e: BuilderError) -> Self {
        Sh1107gError::Builder(e)
    }
}