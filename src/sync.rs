// sync.rs
use crate::{Sh1107g, cmds, error::Sh1107gError};
use embedded_hal::i2c::I2c;
use heapless::Vec;
use core::fmt::Debug;

impl<I2C, E> Sh1107g<I2C>
where
    I2C: I2c<Error = E>,
    E: Debug + embedded_hal::i2c::Error,
{
    fn send(&mut self, control: u8, data: &[u8]) -> Result<(), Sh1107gError<E>> {
        let mut payload = Vec::<u8, 32>::new();
        payload.push(control).map_err(|_| Sh1107gError::PayloadOverflow)?;
        payload.extend_from_slice(data).map_err(|_| Sh1107gError::PayloadOverflow)?;
        self.i2c.write(self.address, &payload).map_err(Sh1107gError::I2cError)
    }

    pub fn send_cmd(&mut self, cmd: &[u8]) -> Result<(), Sh1107gError<E>> {
        self.send(0x80, cmd)
    }

    pub fn init(&mut self) -> Result<(), Sh1107gError<E>> {
        let init_cmds: &[&[u8]] = &[
            cmds::DISPLAY_OFF,
            &cmds::contrast(0x2F),
            &cmds::set_start_line(0x00),
            &cmds::display_offset(0x60),
            cmds::SEGMENT_REMAP_REMAPPED,
            cmds::COM_SCAN_NORMAL,
            cmds::ENTIRE_DISPLAY_RESUME,
            cmds::INVERT_NORMAL,
            cmds::DISPLAY_ON,
        ];

        for cmd in init_cmds {
            self.send(0x80, cmd)?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
        let page_count = crate::DISPLAY_HEIGHT / 8;
        let page_width = crate::DISPLAY_WIDTH;

        for page in 0..page_count {
            self.send_cmd(&[0xB0 + page as u8])?;
            self.send_cmd(&[0x00])?;
            self.send_cmd(&[0x10])?;

            let start = page * page_width;
            let end = start + page_width;

            let mut page_buf = Vec::<u8, { crate::DISPLAY_WIDTH }>::new();
            page_buf.extend_from_slice(&self.buffer[start..end])
                .map_err(|_| Sh1107gError::PayloadOverflow)?;

            for chunk in page_buf.chunks(32 - 1) {
                self.send(0x40, chunk)?;
            }
        }
        Ok(())
    }
}
