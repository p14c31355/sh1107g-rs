use heapless::Vec;
use core::fmt::Debug;

#[cfg(feature = "debug_log")]
use dvcdbg::logger::Logger;

use crate::{error::{BuilderError, Sh1107gError},  Sh1107gBuilder, Sh1107g};
use embedded_hal::i2c::I2c;

impl<'a, I2C, L, E> Sh1107gBuilder<'a, I2C, L>
where
    I2C: I2c<Error = E>,
    L: Logger + 'a,
    E: Debug,
    Sh1107gError<E>: From<E>,
{
    pub fn build_logger(self) -> Result<Sh1107g<'a, I2C, L>, Sh1107gError<E>> {
        let i2c = self.i2c.ok_or(Sh1107gError::Builder(BuilderError::NoI2cConnected))?;
        let mut oled = Sh1107g::new(i2c, self.address, self.logger);

        oled.init()?;
        Ok(oled)
    }
}

impl<'a, I2C, L, E> Sh1107g<'a, I2C, L>
where
    I2C: I2c<Error = E>,
    L: Logger + 'a,
    E: Debug,
{
    /// 共通I2C送信＋ロギング関数。制御バイト（0x80, 0x40など）＋データ配列を送る。
    fn send(&mut self, control: u8, data: &[u8]) -> Result<(), Sh1107gError<E>> {
        let mut payload = Vec::<u8, 65>::new();
        payload.push(control).map_err(|_| Sh1107gError::PayloadOverflow)?;
        payload.extend_from_slice(data).map_err(|_| Sh1107gError::PayloadOverflow)?;

        let res = self.i2c.write(self.address, &payload);

        if let Some(logger) = self.logger.as_mut() {
            logger.log_bytes("i2c_write", &payload);
            logger.log_i2c("i2c_status", res.as_ref().map(|_| ()).map_err(|_| ()));
        }

        res.map_err(Sh1107gError::I2cError)
    }

    /// 単一コマンド送信は制御バイト0x80でsendを呼ぶだけに。
    pub fn send_cmd(&mut self, cmd: u8) -> Result<(), Sh1107gError<E>> {
        self.send(0x80, &[cmd])
    }

    /// 初期化コマンド送信（cmds.rsのINIT_SEQUENCEを使う想定）
    pub fn init(&mut self) -> Result<(), Sh1107gError<E>> {
        use crate::cmds;

        let mut payload = heapless::Vec::<u8, 40>::new();
        payload.push(0x00).map_err(|_| Sh1107gError::PayloadOverflow)?;
        payload.extend_from_slice(cmds::INIT_SEQUENCE).map_err(|_| Sh1107gError::PayloadOverflow)?;

        let res = self.i2c.write(self.address, &payload);
        if let Some(logger) = self.logger.as_mut() {
            logger.log_bytes("init_sequence", &payload);
            log_init_sequence(*logger);
        }
        res.map_err(Sh1107gError::I2cError)?;
        Ok(())
    }

    /// バッファをページごとに送信。送信は全てsend()を経由し、ログも一括。
    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
        use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

        let page_count = DISPLAY_HEIGHT as usize / 8;
        let page_width = DISPLAY_WIDTH as usize;

        for page in 0..page_count {
            // ページアドレスセット（cmdsで型化予定）
            self.send_cmd(0xB0 + page as u8)?;
            self.send_cmd(0x00)?; // カラムアドレス下位
            self.send_cmd(0x10)?; // カラムアドレス上位

            let start = page * page_width;
            let end = start + page_width;
            let page_data = &self.buffer[start..end];

            for chunk in page_data.chunks(64) {
                self.send(0x40, chunk)?;
            }
        }

        Ok(())
    }
}
