// src/error.rs
// ドライバのエラー型を定義
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
    InitError, // From<()> のために追加
}

impl<I2cE> From<I2cE> for Sh1107gError<I2cE> {
    fn from(e: I2cE) -> Self {
        Sh1107gError::I2cError(e)
    }
}

// From<()> の実装を明示的にし、I2cEが()の場合でも適用できるようにする
impl From<()> for Sh1107gError<()> {
    fn from(_: ()) -> Self {
        Sh1107gError::InitError
    }
}