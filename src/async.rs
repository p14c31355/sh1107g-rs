use embedded_hal_async::i2c::I2c; // async版のI2cトレイト

#[cfg(feature = "async")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    pub fn build(self) -> Result<Sh1107g<I2C>, BuilderError> {
        let i2c = self.i2c.ok_or(BuilderError::NoI2cConnected)?;
        let oled = Sh1107g::new(i2c, self.address);
        Ok(oled)
    }
}