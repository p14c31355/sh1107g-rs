use heapless::Vec;
use core::fmt::Debug;
use crate::{Sh1107g, error::Sh1107gError};
use embedded_hal::i2c::I2c;

impl<I2C, E> Sh1107g<I2C>
where
    I2C: I2c<Error = E>,
    E: Debug + embedded_hal::i2c::Error,
{
    /// 共通I2C送信関数
    fn send(&mut self, control: u8, data: &[u8]) -> Result<(), Sh1107gError<E>> {
        // UNO I2Cバッファ制限32バイトを考慮
        for chunk in data.chunks(crate::I2C_MAX_WRITE - 1) {
            let mut payload: Vec<u8, {crate::I2C_MAX_WRITE}> = Vec::new();
            payload.push(control).map_err(|_| Sh1107gError::PayloadOverflow)?;
            payload.extend_from_slice(chunk).map_err(|_| Sh1107gError::PayloadOverflow)?;
            self.i2c.write(self.address, &payload).map_err(Sh1107gError::I2cError)?;
        }
        Ok(())
    }

    /// 単一コマンド送信
    pub fn send_cmd(&mut self, cmd: u8) -> Result<(), Sh1107gError<E>> {
        self.send(0x80, &[cmd])
    }

    /// 初期化コマンド送信
    pub fn init(&mut self) -> Result<(), Sh1107gError<E>> {
        use crate::cmds::*;

        let init_cmds: &[&[u8]] = &[
            &[DisplayPower::Off as u8],
            &SetStartLine(0x00).to_bytes(),
            &Contrast(0x2F).to_bytes(),
            &[SegmentRemap::Remap as u8],
            &[ComOutputScanDirection::Normal as u8],
            &[EntireDisplay::Resume as u8],
            &[Invert::Normal as u8],
            &MultiplexRatio(0x7F).to_bytes(),
            &SetDisplayOffset(0x60).to_bytes(),
            &SetClockDiv { divide_ratio: 0x01, oscillator_freq: 0x01 }.to_bytes(),
            &PreChargePeriod(0x22).to_bytes(),
            &VcomhDeselectLevel(0x35).to_bytes(),
            &ChargePump(true).to_bytes(),
            &[DisplayPower::On as u8],
        ];

        for cmd_bytes in init_cmds {
            self.send(0x80, cmd_bytes)?;
        }

        Ok(())
    }

    /// バッファをOLEDに送信（ページ単位）
    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
        use crate::{DISPLAY_WIDTH, DISPLAY_HEIGHT, COLUMN_OFFSET, I2C_MAX_WRITE};
        let page_count = DISPLAY_HEIGHT / 8;

        for page in 0..page_count {
            self.send_cmd(0xB0 + page as u8)?;
            self.send_cmd(0x00 + COLUMN_OFFSET as u8)?;
            self.send_cmd(0x10)?;

            let start = page * DISPLAY_WIDTH;
            let end = start + DISPLAY_WIDTH;

            // コピーを作ることで immutable borrow を避ける
            let mut page_buf = heapless::Vec::<u8, { DISPLAY_WIDTH }>::new();
            page_buf.extend_from_slice(&self.buffer[start..end])
                .map_err(|_| Sh1107gError::PayloadOverflow)?;

            for chunk in page_buf.chunks(I2C_MAX_WRITE - 1) {
                self.send(0x40, chunk)?;
            }
        }

        Ok(())
    }
}
