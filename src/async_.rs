// src/async_.rs
//! async

#[cfg(feature = "async_")]
use crate::error::{Sh1107gError, BuilderError};

#[cfg(feature = "async_")]
use crate::{Sh1107g, Sh1107gBuilder};

#[cfg(feature = "async_")]
use core::result::{Result, Result::Ok};

// `Sh1107gBuilder` の impl ブロックのジェネリクスを修正
// `'a` と `L` を型パラメータとして明示的に指定
#[cfg(feature = "async_")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
    Sh1107gError<E>: From<E>,
{
    pub async fn build_async(self) -> Result<Sh1107g<I2C>, Sh1107gError<E>>{
        let i2c = self.i2c.ok_or(Sh1107gError::Builder(BuilderError::NoI2cConnected))?;

        let mut oled = Sh1107g::new(i2c, self.address);

        if let Err(_e) = oled.init_async().await {
            return Err(Sh1107gError::Builder(BuilderError::InitFailed));
        }

        Ok(oled)
    }
}

// `Sh1107g` の impl ブロックのジェネリクスを修正
#[cfg(feature = "async_")]
impl<I2C, E> Sh1107g<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    async fn send_cmd_async(&mut self, cmd: u8) -> Result<(), E> {
        let payload = [0x80, cmd];
        self.i2c.write(self.address, &payload).await
    }

    pub async fn init_async(&mut self) -> Result<(), Sh1107gError<E>>{
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