/// sync
#[cfg(feature = "sync")]
use embedded_hal::i2c::I2c;

#[cfg(feature = "sync")]
use crate::Sh1107gError;

#[cfg(feature = "sync")]
use crate::{BuilderError, Sh1107g, Sh1107gBuilder};

#[cfg(feature = "sync")]
use core::result::Result;
#[cfg(feature = "sync")]
use core::result::Result::Ok;

// Sh1107g instance ( builded by builder ) call init and flush
#[cfg(feature = "sync")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    pub fn build(
        self,
        serial: &mut dyn core::fmt::Write,
    ) -> Result<Sh1107g<I2C>, Sh1107gError<E>> {
        writeln!(serial, "BUILD START").ok();

        let i2c = self.i2c.ok_or(Sh1107gError::Builder(BuilderError::NoI2cConnected))?;
        writeln!(serial, "I2C CONNECTED").ok();

        let mut oled = Sh1107g::new(i2c, self.address);
        writeln!(serial, "DRIVER CREATED").ok();

        match oled.init() {
            Ok(_) => writeln!(serial, "INIT OK").ok(),
            Err(_) => writeln!(serial, "INIT FAILED").ok(),
        };

        Ok(oled)
    }
}

// let size = self.size.ok_or(BuilderError::NoDisplaySizeDefined)?; // サイズが必須の場合
        // If you need, more add configure
// size: size,
            // rotation: self.rotation,// Sh1107g::new init include buffer
            // display initialize やるかどうか

// Sh1107g impl block
#[cfg(feature = "sync")]
impl<I2C, E> Sh1107g<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    // コマンドを単独で送信するヘルパー関数
    fn send_cmd(&mut self, cmd: u8) -> Result<(), E> {
        let payload = [0x80, cmd]; // コントロールバイト0x80を付加
        self.i2c.write(self.address, &payload)
    }

    // 複数のコマンドをセットで送信するヘルパー関数
    // send_cmds の push エラーを独自に変換
    fn send_cmds(&mut self, cmds: &[u8]) -> Result<(), E> {
        use heapless::Vec;
        let mut payload = Vec::<u8, 20>::new();
        payload.push(0x80).map_err(|_| /* 適切なエラーに変換。例えばpanicや独自エラー */ panic!("payload overflow"))?;
        payload.extend_from_slice(cmds).map_err(|_| panic!("payload overflow"))?;
        self.i2c.write(self.address, &payload)
    }

    /// Init display (U8g2ライブラリ準拠)
    pub fn init(&mut self) -> Result<(), E> {
        use heapless::Vec;
        let init_cmds: &[u8] = &[
            0xAE,           // Display Off
            0x40,           // Display Start Line
            0x20, 0x02,     // Memory Addressing Mode
            0x81, 0x80,     // Contrast Control
            0xA0,           // Segment Remap (通常表示)
            0xA4,           // Entire Display On
            0xA6,           // Normal Display
            0xA8, 0x7F,     // Multiplex Ratio
            0xD3, 0x60,     // Display Offset
            0xD5, 0x51,     // Display Clock Divide Ratio
            0xC0,           // COM Output Scan Direction (通常表示)
            0xD9, 0x22,     // Pre-charge Period
            0xDA, 0x12,     // COM Pins Hardware Configuration
            0xDB, 0x35,     // VCOMH Deselect Level
            0xAD, 0x8B,     // Charge Pump
            0xAF,           // Display On
        ];

        let mut payload = Vec::<u8, 34>::new();
        payload.push(0x00).map_err(|_| panic!("payload overflow"))?;
        payload.extend_from_slice(init_cmds).map_err(|_| panic!("payload overflow"))?;
        self.i2c.write(self.address, &payload)?;

        Ok(())
    }

    /// Rendering
    pub fn flush(&mut self) -> Result<(), Sh1107gError<E>> {
    use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
    use heapless::Vec;

    let page_count = DISPLAY_HEIGHT as usize / 8;
    let page_width = DISPLAY_WIDTH as usize;

    for page in 0..page_count {
            self.send_cmd(0xB0 + page as u8)?;
            self.send_cmd(0x00)?;
            self.send_cmd(0x10)?;

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