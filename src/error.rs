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

// src/main.rs または専用のerrorsモジュールに記述
#[derive(Debug)]
pub enum AppError<E> where E: embedded_hal::i2c::Error {
    I2cError(E), // I2Cエラーをラップする
    BuilderError,                     // `()`から発生するエラー用
    // 必要に応じて他のエラー型を追加
}

// `E: From<()>` の要件を満たすために From<()> を実装
impl<E> From<()> for AppError<E>
where
    E: embedded_hal::i2c::Error,
{
    fn from(_: ()) -> Self {
        AppError::BuilderError
    }
}