// src/sync.rs
/// sync

#[cfg(feature = "sync")]
use crate::error::{Sh1107gError, BuilderError};

#[cfg(feature = "sync")]
use crate::{Sh1107g, Sh1107gBuilder};

#[cfg(feature = "sync")]
use core::result::{Result, Result::Ok};

#[cfg(feature = "sync")]
use ufmt::uWrite;

// Sh1107g instance ( builded by builder ) call init and flush
#[cfg(feature = "sync")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
    Sh1107gError<E>: From<E>,
{
    pub fn build(
        self,
        serial: &mut dyn ufmt::uWrite<Error = core::convert::Infallible>,
    ) -> Result<Sh1107g<I2C>, Sh1107gError<E>>
    where
    I2C: uWrite<Error = E>,
    E: ufmt::uDebug,
    {

        let i2c = self.i2c.ok_or(Sh1107gError::Builder(BuilderError::NoI2cConnected))?;

        let mut oled = {
            #[cfg(feature = "debug_log")]
            {
                match self.logger {
                    Some(logger) => Sh1107g::new_with_logger(i2c, self.address, logger),
                    None => Sh1107g::new(i2c, self.address),
                }
            }

            #[cfg(not(feature = "debug_log"))]
            {
                Sh1107g::new(i2c, self.address)
            }
        };

    if let Err(e) = oled.init() {
        return Err(Sh1107gError::Builder(BuilderError::InitFailed));
    } else {
    }
        Ok(oled)
    }
}

// Sh1107g impl block
#[cfg(feature = "sync")]
impl<I2C, E> Sh1107g<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    // コマンドを単独で送信するヘルパー関数
    fn send_cmd(&mut self, cmd: u8) -> Result<(), E> {

        #[cfg(feature = "debug_log")]
        if let Some(logger) = self.logger.as_mut() {
            use heapless::String;
            let mut msg = String::<16>::new();
            let _ = ufmt::uwrite!(&mut msg, "CMD = 0x{:02X}", cmd);
            logger.log(&msg);
        }

        let payload = [0x80, cmd]; // コントロールバイト0x80を付加
        self.i2c.write(self.address, &payload)
    }

    // 複数のコマンドをセットで送信するヘルパー関数
    // send_cmds の push エラーを独自に変換
    /*
    fn send_cmds(&mut self, cmds: &[u8]) -> Result<(), Sh1107gError<E>> {
        use heapless::Vec;
        let mut payload = Vec::<u8, 20>::new();
        payload.push(0x80).map_err(|_| Sh1107gError::PayloadOverflow)?;
        payload.extend_from_slice(cmds).map_err(|_| Sh1107gError::PayloadOverflow)?;
        self.i2c.write(self.address, &payload).map_err(Sh1107gError::I2cError)
    }
    */

    /// Init display (U8g2ライブラリ準拠)
    pub fn init(&mut self) -> Result<(), Sh1107gError<E>>
    where
    I2C: uWrite<Error = E>,
    E: ufmt::uDebug,
    {
        let init_cmds: &[u8] = &[
            0xAE, 0x40, 0x20, 0x02, 0x81, 0x80, 0xA0, 0xA4,
            0xA6, 0xA8, 0x7F, 0xD3, 0x60, 0xD5, 0x51, 0xC0,
            0xD9, 0x22, 0xDA, 0x12, 0xDB, 0x35, 0xAD, 0x8B,
            0xAF,
        ];

        // 1. payloadの作成
        let mut payload = heapless::Vec::<u8, 40>::new(); // ←サイズ保険

        // 2. push(0x00)
        payload.push(0x00).map_err(|_| {
            Sh1107gError::PayloadOverflow
        })?;

        payload.extend_from_slice(init_cmds).map_err(|_| {
            Sh1107gError::PayloadOverflow
        })?;

        self.i2c.write(self.address, &payload).map_err(|e| {
            Sh1107gError::I2cError(e)
        })?;

        Ok(())
    }

    pub fn write_command_list<W: uWrite>(
        &mut self,
        cmds: &[u8],
        serial: &mut W,
    ) -> Result<(), E> {
        for (i, &cmd) in cmds.iter().enumerate() {
            // ログ出力
            // let _ = uwriteln!(serial, "CMD[{}] = 0x{:02X}", i, cmd);

            // コマンド送信
            self.i2c.write(self.address, &[0x00, cmd])?;
        }
        Ok(())
    }

    /// Rendering
    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
    use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

    let page_count = DISPLAY_HEIGHT as usize / 8;
    let page_width = DISPLAY_WIDTH as usize;

    for page in 0..page_count {
            // send_cmdがResult<(), E>を返すため、`map_err`で変換が必要
            self.send_cmd(0xB0 + page as u8).map_err(Sh1107gError::I2cError)?;
            self.send_cmd(0x00).map_err(Sh1107gError::I2cError)?;
            self.send_cmd(0x10).map_err(Sh1107gError::I2cError)?;

        let start_index = page * page_width;
        let end_index = start_index + page_width;
        let page_data = &self.buffer[start_index..end_index];

        for chunk in page_data.chunks(64) {
        let mut payload = heapless::Vec::<u8, {1 + 64}>::new();
        payload.push(0x40).map_err(|_| Sh1107gError::PayloadOverflow)?;
        payload.extend_from_slice(chunk).map_err(|_| Sh1107gError::PayloadOverflow)?;
        self.i2c.write(self.address, &payload).map_err(Sh1107gError::I2cError)?;
        }
    }
        Ok(())
    }
}