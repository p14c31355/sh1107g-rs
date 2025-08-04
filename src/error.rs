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

impl<I2cE> From<BuilderError> for Sh1107gError<I2cE> {
    fn from(e: BuilderError) -> Self {
        Sh1107gError::Builder(e)
    }
}

impl<I2cE> From<I2cE> for Sh1107gError<I2cE>
where
    I2cE: embedded_hal::i2c::Error,
{
    fn from(e: I2cE) -> Self {
        Sh1107gError::I2cError(e)
    }
}
