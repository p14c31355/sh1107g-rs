// src/async_.rs
//! async
#[cfg(feature = "debug_log")]
use dvcdbg::logger::Logger;

#[cfg(feature = "async_")]
use crate::error::{Sh1107gError, BuilderError};

#[cfg(feature = "debug_log")]
use crate::cmds::log_init_sequence;

#[cfg(feature = "async_")]
use crate::{Sh1107g, Sh1107gBuilder};

#[cfg(feature = "async_")]
use core::result::{Result, Result::Ok};

// `Sh1107gBuilder` の impl ブロックのジェネリクスを修正
// `'a` と `L` を型パラメータとして明示的に指定
#[cfg(feature = "async_")]
impl<'a, I2C, L, E> Sh1107gBuilder<'a, I2C, L>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>, // 非同期版I2Cトレイト
    L: Logger + 'a,
    E: core::fmt::Debug,
    Sh1107gError<E>: From<E>,
{
    pub async fn build_async(self) -> Result<Sh1107g<'a, I2C, L>, Sh1107gError<E>>{
        let i2c = self.i2c.ok_or(Sh1107gError::Builder(BuilderError::NoI2cConnected))?;

        let mut oled = Sh1107g::new(i2c, self.address, self.logger);

        if let Err(_e) = oled.init().await {
            return Err(Sh1107gError::Builder(BuilderError::InitFailed));
        }

        Ok(oled)
    }
}

// `Sh1107g` の impl ブロックのジェネリクスを修正
#[cfg(feature = "async_")]
impl<'a, I2C, L, E> Sh1107g<'a, I2C, L>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>, // 非同期版I2Cトレイト
    L: Logger + 'a,
    E: core::fmt::Debug,
{
    async fn send_cmd_async(&mut self, cmd: u8) -> Result<(), E> {
        #[cfg(feature = "debug_log")]
        if let Some(logger) = self.logger.as_mut() {
            use core::fmt::Write;
            use heapless::String;
            let mut msg = String::<16>::new();
            let _ = write!(&mut msg, "CMD = 0x{:02X}", cmd);
            (*logger).log(&msg);
        }

        let payload = [0x80, cmd];
        self.i2c.write(self.address, &payload).await
    }

    pub async fn init_async(&mut self) -> Result<(), Sh1107gError<E>>{
        let init_cmds: &[u8] = &[
            0xAE, 0x40, 0x20, 0x02, 0x81, 0x80, 0xA0, 0xA4,
            0xA6, 0xA8, 0x7F, 0xD3, 0x60, 0xD5, 0x51, 0xC0,
            0xD9, 0x22, 0xDA, 0x12, 0xDB, 0x35, 0xAD, 0x8B,
            0xAF,
        ];

        let mut payload = heapless::Vec::<u8, 40>::new();
        payload.push(0x00).map_err(|_| {
            Sh1107gError::PayloadOverflow
        })?;

        payload.extend_from_slice(init_cmds).map_err(|_| {
            Sh1107gError::PayloadOverflow
        })?;

        self.i2c.write(self.address, &payload).await.map_err(|e| {
            Sh1107gError::I2cError(e)
        })?;

        #[cfg(feature = "debug_log")]
        if let Some(logger) = self.logger.as_mut() {
            log_init_sequence(*logger);
        }

        Ok(())
    }

    pub async fn flush_async(&mut self) -> Result<(), Sh1107gError<E>> {
        use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

        let page_count = DISPLAY_HEIGHT as usize / 8;
        let page_width = DISPLAY_WIDTH as usize;

        for page in 0..page_count {
            self.send_cmd_async(0xB0 + page as u8).await.map_err(Sh1107gError::I2cError)?;
            self.send_cmd_async(0x00).await.map_err(Sh1107gError::I2cError)?;
            self.send_cmd_async(0x10).await.map_err(Sh1107gError::I2cError)?;

            let start_index = page * page_width;
            let end_index = start_index + page_width;
            let page_data = &self.buffer[start_index..end_index];

            for chunk in page_data.chunks(64) {
                let mut payload = heapless::Vec::<u8, {1 + 64}>::new();
                payload.push(0x40).map_err(|_| Sh1107gError::PayloadOverflow)?;
                payload.extend_from_slice(chunk).map_err(|_| Sh1107gError::PayloadOverflow)?;
                self.i2c.write(self.address, &payload).await.map_err(Sh1107gError::I2cError)?;
            }
        }
        Ok(())
    }
}