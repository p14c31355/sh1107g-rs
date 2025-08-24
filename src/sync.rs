use crate::{Sh1107g, cmds, error::Sh1107gError};
use embedded_hal::i2c::I2c;
use heapless::Vec;
use core::fmt::Debug;

impl<I2C, E> Sh1107g<I2C>
where
    I2C: I2c<Error = E>,
    E: Debug,
{
    /// I2C 送信
    fn send(&mut self, control: u8, data: &[u8]) -> Result<(), Sh1107gError<E>> {
        let mut payload = Vec::<u8, 32>::new();
        payload.push(control).map_err(|_| Sh1107gError::PayloadOverflow)?;
        payload.extend_from_slice(data).map_err(|_| Sh1107gError::PayloadOverflow)?;
        self.i2c.write(self.address, &payload).map_err(Sh1107gError::I2cError)
    }

    /// 初期化
    pub fn init(&mut self) -> Result<(), Sh1107gError<E>> {
        let init_cmds: &[&[u8]] = &[
            cmds::DISPLAY_OFF,
            &cmds::set_start_line(0x00),
            cmds::CONTRAST,
            cmds::SEGMENT_REMAP_REMAP,
            cmds::COM_SCAN_NORMAL,
            cmds::ENTIRE_DISPLAY_RESUME,
            cmds::INVERT_NORMAL,
            &cmds::multiplex_ratio(0x7F),
            &cmds::display_offset(0x60),
            &cmds::clock_div(0x01, 0x01),
            &cmds::precharge_period(0x22),
            &cmds::vcomh_deselect(0x35),
            &cmds::charge_pump(true),
            cmds::DISPLAY_ON,
        ];

        for cmd in init_cmds {
            self.send(0x80, cmd)?;
        }
        Ok(())
    }

    /// バッファを送信（ページ単位）
    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
        let page_count = 128 / 8; // DISPLAY_HEIGHT / 8
        let page_width = 128;     // DISPLAY_WIDTH

        for page in 0..page_count {
            self.send(0x80, &[0xB0 + page as u8])?;
            self.send(0x80, &[0x00])?;
            self.send(0x80, &[0x10])?;

            let start = page * page_width;
            let end = start + page_width;

            // ページごとに Vec にコピーして借用を避ける
            let mut page_data = Vec::<u8, 128>::new();
            page_data.extend_from_slice(&self.buffer[start..end])
                .map_err(|_| Sh1107gError::PayloadOverflow)?;

            for chunk in page_data.chunks(32 - 1) { // I2C_MAX_WRITE - 1
                self.send(0x40, chunk)?;
            }
        }

        Ok(())
    }
}
