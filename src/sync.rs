// sync.rs

use crate::{Sh1107g, cmds::SH1107G_INIT_CMDS, error::Sh1107gError};
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
        self.send(0x00, cmd)
    }

    pub fn init(&mut self) -> Result<(), Sh1107gError<E>> {
        for cmd in SH1107G_INIT_CMDS {
            self.send_cmd(cmd)?;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
        let page_count = crate::DISPLAY_HEIGHT / 8;
        let page_width = crate::DISPLAY_WIDTH;

        for page in 0..page_count {
            self.send_cmd(&[0xB0 | page as u8])?;
            self.send_cmd(&[0x00 | (crate::COLUMN_OFFSET & 0x0F) as u8])?;
            self.send_cmd(&[0x10 | ((crate::COLUMN_OFFSET >> 4) & 0x0F) as u8])?;

            let start = page * page_width;
            let end = start + page_width;
            
            let mut temp_page_buffer: heapless::Vec<u8, { crate::DISPLAY_WIDTH }> = heapless::Vec::new();
            temp_page_buffer.extend_from_slice(&self.buffer[start..end])
                .map_err(|_| Sh1107gError::PayloadOverflow)?;

            for chunk in temp_page_buffer.chunks(crate::I2C_MAX_WRITE - 1) {
                self.send(0x40, chunk)?;
            }
        }
        Ok(())
    }
}
