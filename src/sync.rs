use embedded_hal::i2c::I2c;

// Sh1107g instance ( builded by builder ) call init and flush
#[cfg(feature = "sync")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Build Sh1107g instance
    pub fn build(self) -> Result<Sh1107g<I2C>, BuilderError> {
        let i2c = self.i2c.ok_or(BuilderError::NoI2cConnected)?;
        // let size = self.size.ok_or(BuilderError::NoDisplaySizeDefined)?; // サイズが必須の場合

        // If you need, more add configure

        let oled = Sh1107g::new(i2c, self.address
            // size: size,
            // rotation: self.rotation,
            ); // Sh1107g::new init include buffer

        // ディスプレイの初期化を自動的に行っても良いし、build() はインスタンスの作成のみに責任を持ち、
        // init() は別途呼び出すようにしても良い。今回はシンプルにインスタンス作成まで。
        Ok(oled)
    }
}