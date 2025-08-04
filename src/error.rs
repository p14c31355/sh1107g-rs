// src/error.rs
// ドライバのエラー型を定義
#[derive(Debug)]
pub enum BuilderError {
    NoI2cConnected,
    InitFailed,
}

#[derive(Debug)]
pub enum Sh1107gError<I2cE> {
    Builder(BuilderError),
    PayloadOverflow,
    I2cError(I2cE),
}

// I2CエラーをSh1107gErrorに変換するための汎用的なFrom実装
// `I2cE`が`()`でない場合を想定
impl<I2cE> From<I2cE> for Sh1107gError<I2cE> {
    fn from(e: I2cE) -> Self {
        Sh1107gError::I2cError(e)
    }
}

// BuilderErrorをSh1107gErrorに変換するためのFrom実装
// `Sh1107gBuilder`の`build`メソッドで`?`演算子を使用するために必要
impl<I2cE> From<BuilderError> for Sh1107gError<I2cE> {
    fn from(e: BuilderError) -> Self {
        Sh1107gError::Builder(e)
    }
}