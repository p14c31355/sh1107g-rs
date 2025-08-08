    use heapless::String;
    use core::fmt::Write; // fmt::Write トレイトを使う

#[cfg(feature = "debug_log")]
use dvcdbg::logger::{Logger, log_cmd};

#[cfg(feature = "sync")]
use crate::error::{Sh1107gError, BuilderError};

#[cfg(feature = "debug_log")]
use crate::cmds::log_init_sequence;

#[cfg(feature = "sync")]
use crate::{Sh1107g, Sh1107gBuilder};

#[cfg(feature = "sync")]
use core::result::Result;

#[cfg(feature = "sync")]
impl<'a, I2C, L, E> Sh1107gBuilder<'a, I2C, L>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    L: Logger + 'a,
    E: core::fmt::Debug,
    Sh1107gError<E>: From<E>,
{
    pub fn build_logger(self) -> Result<Sh1107g<'a, I2C, L>, Sh1107gError<E>> {
        let i2c = self.i2c.ok_or(Sh1107gError::Builder(BuilderError::NoI2cConnected))?;
        let mut oled = Sh1107g::new(i2c, self.address, self.logger);

        if let Err(_) = oled.init() {
            return Err(Sh1107gError::Builder(BuilderError::InitFailed));
        }
        Ok(oled)
    }
}

#[cfg(feature = "sync")]
impl<'a, I2C, L, E> Sh1107g<'a, I2C, L>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    L: Logger + 'a,
    E: core::fmt::Debug,
{
    /// 単一コマンド送信
    pub fn send_cmd(&mut self, cmd: u8) -> Result<(), E> {
        let res = self.i2c.write(self.address, &[0x80, cmd]);

        if let Some(logger) = self.logger.as_mut() {
            let mut buf: String<64> = String::new();
            let _ = write!(buf, "send_cmd: 0x{:02X}", cmd);
            logger.log_i2c(buf.as_str(), res.as_ref().map(|_| ()).map_err(|_| ()));
        }

        res
    }

    /// コマンド送信（write_command用）
    fn write_command(&mut self, cmd: u8) -> Result<(), E> {
        let res = self.i2c.write(self.address, &[cmd]);
        if let Some(logger) = self.logger.as_mut() {
            log_cmd(*logger, cmd);
            (*logger).log_i2c("write_command", res.as_ref().map(|_| ()).map_err(|_| ()));
        }
        res
    }

    /// 初期化
    pub fn init(&mut self) -> Result<(), Sh1107gError<E>> {
        let init_cmds: &[u8] = &[
        0xAE,       // DISPLAY_OFF
        0xAD, 0x8B, // CHARGE_PUMP_ON_CMD + CHARGE_PUMP_ON_DATA （チャージポンプONは早めに）
        0xA8, 0x7F, // SET_MULTIPLEX_RATIO + MULTIPLEX_RATIO_DATA
        0xD3, 0x60, // DISPLAY_OFFSET_CMD + DISPLAY_OFFSET_DATA
        0x40,       // DISPLAY_START_LINE_CMD
        0xD5, 0x51, // CLOCK_DIVIDE_CMD + CLOCK_DIVIDE_DATA
        0xC0,       // COM_OUTPUT_SCAN_DIR
        0xDA, 0x12, // SET_COM_PINS_CMD + SET_COM_PINS_DATA
        0x81, 0x80, // CONTRAST_CONTROL_CMD + CONTRAST_CONTROL_DATA
        0xD9, 0x22, // PRECHARGE_CMD + PRECHARGE_DATA
        0xDB, 0x35, // VCOM_DESELECT_CMD + VCOM_DESELECT_DATA
        0xA0,       // SEGMENT_REMAP
        0xA4,       // SET_ENTIRE_DISPLAY_ON_OFF_CMD
        0xA6,       // SET_NORMAL_INVERSE_DISPLAY_CMD
        0xAF,       // DISPLAY_ON
    ];

        let mut payload = heapless::Vec::<u8, 40>::new();
        payload.push(0x00).map_err(|_| Sh1107gError::PayloadOverflow)?;
        payload.extend_from_slice(init_cmds).map_err(|_| Sh1107gError::PayloadOverflow)?;

        let res = self.i2c.write(self.address, &payload);
        if let Some(logger) = self.logger.as_mut() {
            (*logger).log_i2c("init_sequence", res.as_ref().map(|_| ()).map_err(|_| ()));
            log_init_sequence(*logger);
        }
        res.map_err(Sh1107gError::I2cError)?;
        Ok(())
    }

    /// バッファ内容をOLEDへ送信
    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
        use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
        let page_count = DISPLAY_HEIGHT as usize / 8;
        let page_width = DISPLAY_WIDTH as usize;

        for page in 0..page_count {
            self.send_cmd(0xB0 + page as u8).map_err(Sh1107gError::I2cError)?;
            self.send_cmd(0x00).map_err(Sh1107gError::I2cError)?;
            self.send_cmd(0x10).map_err(Sh1107gError::I2cError)?;

            let start_index = page * page_width;
            let end_index = start_index + page_width;
            let page_data = &self.buffer[start_index..end_index];

            for chunk in page_data.chunks(64) {
                let mut payload = heapless::Vec::<u8, { 1 + 64 }>::new();
                payload.push(0x40).map_err(|_| Sh1107gError::PayloadOverflow)?;
                payload.extend_from_slice(chunk).map_err(|_| Sh1107gError::PayloadOverflow)?;
                let res = self.i2c.write(self.address, &payload);
                if let Some(logger) = self.logger.as_mut() {
                    (*logger).log_i2c("flush_chunk", res.as_ref().map(|_| ()).map_err(|_| ()));
                }
                res.map_err(Sh1107gError::I2cError)?;
            }
        }
        Ok(())
    }
}
