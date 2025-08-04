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
}

// From 実装で `?` を使えるように、I2Cエラーをラップする
// ただし、I2cE がユニット型 () の場合はこの実装を適用しない
impl<I2cE> From<I2cE> for Sh1107gError<I2cE>
where
    I2cE: core::fmt::Debug, // 必要に応じてI2cEに制約を追加
{
    fn from(e: I2cE) -> Self {
        Sh1107gError::I2cError(e)
    }
}


// `E: From<()>` の要件を満たすために From<()> を実装
// この実装は、E0599エラーを解決するために必要です。
impl<I2cE> From<()> for Sh1107gError<I2cE> {
    fn from(_: ()) -> Self {
        // ビルダーのエラーを返すようにする
        Sh1107gError::Builder(BuilderError::InitFailed)
    }
}