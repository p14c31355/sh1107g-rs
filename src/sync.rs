use heapless::Vec;
use core::fmt::Debug;

use crate::{error::Sh1107gError,  Sh1107g};
use embedded_hal::i2c::I2c;

impl<I2C, E> Sh1107g<I2C>
where
    I2C: I2c<Error = E>,
    E: Debug + embedded_hal::i2c::Error,
{
    /// 共通I2C送信＋ロギング関数。制御バイト（0x80, 0x40など）＋データ配列を送る。
    fn send(&mut self, control: u8, data: &[u8]) -> Result<(), Sh1107gError<E>> {
        let mut payload = Vec::<u8, 65>::new();
        payload.push(control).map_err(|_| Sh1107gError::PayloadOverflow)?;
        payload.extend_from_slice(data).map_err(|_| Sh1107gError::PayloadOverflow)?;

        let res = self.i2c.write(self.address, &payload);

        res.map_err(Sh1107gError::I2cError)
    }

    /// 単一コマンド送信は制御バイト0x80でsendを呼ぶだけに。
    pub fn send_cmd(&mut self, cmd: u8) -> Result<(), Sh1107gError<E>> {
        self.send(0x80, &[cmd])
    }

    /// 初期化コマンド送信（cmds.rsのINIT_SEQUENCEを使う想定）
    pub fn init(&mut self) -> Result<(), Sh1107gError<E>> {
        use crate::cmds::*;

        // INIT_SEQUENCE を &[&[u8]] の形で定義
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
            // &SetComPins(0x12).to_bytes(),
            // Since it is set for the IC at the factory, 
            // it does not need to be explicitly specified during initialisation.
            &SetClockDiv { divide_ratio: 0x01, oscillator_freq: 0x01 }.to_bytes(),
            &PreChargePeriod(0x22).to_bytes(),
            &VcomhDeselectLevel(0x35).to_bytes(),
            &ChargePump(true).to_bytes(),
            &[DisplayPower::On as u8],
        ];

        let mut payload = heapless::Vec::<u8, 64>::new();
        // コントロールバイト 0x00 はコマンド続き送信の意味
        payload.push(0x00).map_err(|_| Sh1107gError::PayloadOverflow)?;

        // コマンド配列を展開してpayloadに連結
        for cmd_bytes in init_cmds.iter() {
            payload.extend_from_slice(cmd_bytes).map_err(|_| Sh1107gError::PayloadOverflow)?;
        }

        let res = self.i2c.write(self.address, &payload);
        res.map_err(Sh1107gError::I2cError)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
        use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

        let page_count = DISPLAY_HEIGHT as usize / 8;
        let page_width = DISPLAY_WIDTH as usize;

        for page in 0..page_count {
            self.send_cmd(0xB0 + page as u8)?;
            self.send_cmd(0x00)?;
            self.send_cmd(0x10)?;

            let start = page * page_width;
            let end = start + page_width;

            // immutable borrow を作らずに所有権コピー
            let mut page_data: heapless::Vec<u8, {DISPLAY_WIDTH as usize}> = 
                heapless::Vec::from_slice(&self.buffer[start..end])
                    .map_err(|_| Sh1107gError::PayloadOverflow)?;

            for chunk in page_data.chunks() {
                self.send(0x40, chunk)?;
            }
        }

        Ok(())
    }
}
