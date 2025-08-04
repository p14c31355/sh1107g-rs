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

impl From<BuilderError> for Sh1107gError<core::convert::Infallible> {
    fn from(e: BuilderError) -> Self {
        Sh1107gError::Builder(e)
    }
}

impl From<embedded_hal::i2c::Error> for Sh1107gError<embedded_hal::i2c::Error> {
    fn from(e: embedded_hal::i2c::Error) -> Self {
        Sh1107gError::I2cError(e)
    }
}
