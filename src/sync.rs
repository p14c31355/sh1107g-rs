use embedded_hal::i2c::I2c;

// BuilderからbuildされたSh1107gインスタンスがinitとflushを呼ぶように変更
#[cfg(feature = "sync")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// 設定に基づきSh1107gインスタンスを構築する。
    pub fn build(self) -> Result<Sh1107g<I2C>, BuilderError> {
        let i2c = self.i2c.ok_or(BuilderError::NoI2cConnected)?;
        // let size = self.size.ok_or(BuilderError::NoDisplaySizeDefined)?; // サイズが必須の場合

        // サイズや回転を設定するオプションを追加した場合、Sh1107g構造体にもそれらのフィールドを追加し、
        // ここで渡す必要があります。

        let oled = Sh1107g::new(i2c, self.address
            // size: size,
            // rotation: self.rotation,
            ); // Sh1107g::newは内部バッファを初期化する

        // ディスプレイの初期化を自動的に行っても良いし、build() はインスタンスの作成のみに責任を持ち、
        // init() は別途呼び出すようにしても良い。今回はシンプルにインスタンス作成まで。
        Ok(oled)
    }
}