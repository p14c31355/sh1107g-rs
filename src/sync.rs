/// sync
#[cfg(feature = "sync")]
use embedded_hal::i2c::I2c;

#[cfg(feature = "sync")]
use crate::{cmds::*, BuilderError, Sh1107g, Sh1107gBuilder, DISPLAY_WIDTH};

#[cfg(feature = "sync")]
use core::result::Result;
#[cfg(feature = "sync")]
use core::result::Result::Ok;

// Sh1107g instance ( builded by builder ) call init and flush
#[cfg(feature = "sync")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Build Sh1107g instance
    pub fn build(self) -> Result<Sh1107g<I2C>, BuilderError> {
        let i2c = self.i2c.ok_or(BuilderError::NoI2cConnected)?;
        // let size = self.size.ok_or(BuilderError::NoDisplaySizeDefined)?; // サイズが必須の場合

        // If you need, more add configure

        let oled = Sh1107g::new(i2c, self.address
            // size: size,
            // rotation: self.rotation,
            ); // Sh1107g::new init include buffer

        // display initialize やるかどうか
        Ok(oled)
    }
}

// Sh1107g impl block
#[cfg(feature = "sync")]
// sync.rs または async.rs

// Sh1107g impl block
impl<I2C, E> Sh1107g<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    // コマンドを単独で送信するヘルパー関数
    fn send_cmd(&mut self, cmd: u8) -> Result<(), E> {
        let payload = [0x80, cmd]; // コントロールバイト0x80を付加
        self.i2c.write(self.address, &payload)
    }

    // 複数のコマンドをセットで送信するヘルパー関数
    // データシートに従い、1つのI2Cトランザクションで送信
    fn send_cmds(&mut self, cmds: &[u8]) -> Result<(), E> {
        let mut payload = heapless::Vec::<u8, 20>::new();
        payload.push(0x80).unwrap();
        payload.extend_from_slice(cmds).unwrap();
        self.i2c.write(self.address, &payload)
    }

    /// Init display (U8g2ライブラリ準拠)
    pub fn init(&mut self) -> Result<(), E> {
        // 全ての初期化コマンドを一つの配列にまとめる
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
        
        let mut payload = heapless::Vec::<u8, 34>::new();
        payload.push(0x00).unwrap(); // 0x00: コマンドモードの制御バイト
        payload.extend_from_slice(init_cmds).unwrap();
        self.i2c.write(self.address, &payload)?;

        Ok(())
    }

    /// Rendering
    // Send self internal buffer
    // flush() 関数全体をこのコードに置き換えてください

    pub fn flush(&mut self) -> Result<(), E> {
        use crate::DISPLAY_HEIGHT;

        let page_count = DISPLAY_HEIGHT as usize / 8;
        let page_width = crate::DISPLAY_WIDTH as usize;

        for page in 0..page_count {
            // ページコマンドは今のままでOK
            self.send_cmd(0xB0 + page as u8)?;
            self.send_cmd(0x00)?;
            self.send_cmd(0x10)?;

            // 128バイトを2回に分けて送信（例：64 + 64）
            let start_index = page * page_width;
            let end_index = start_index + page_width;
            let page_data = &self.buffer[start_index..end_index];

            for chunk in page_data.chunks(64) {
                let mut payload = heapless::Vec::<u8, {1 + 64}>::new();
                payload.push(0x40).unwrap(); // データモード
                payload.extend_from_slice(chunk).unwrap();
                self.i2c.write(self.address, &payload)?;
            }
        }
        Ok(())
    }
}