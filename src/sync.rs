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
        // コントロールバイト0x80 + コマンドバイト群
        let mut payload = heapless::Vec::<u8, 20>::new();
        payload.push(0x80).unwrap();
        payload.extend_from_slice(cmds).unwrap();
        self.i2c.write(self.address, &payload)
    }

    /// Init display (U8g2ライブラリ準拠)
    pub fn init(&mut self) -> Result<(), E> {
        // 1. ディスプレイをオフにする
        self.send_cmd(0xAE)?;

        // 2. Display Start Line
        self.send_cmd(0x40)?;

        // 3. Memory Addressing Mode (Page Addressing Mode)
        self.send_cmds(&[0x20, 0x02])?;

        // 4. Contrast Control
        self.send_cmds(&[0x81, 0x80])?; // ★ ここを修正 ★

        // 5. Segment Remap (通常表示)
        self.send_cmd(0xA0)?; // ★ ここを修正 ★

        // 6. Entire Display On / Off
        self.send_cmd(0xA4)?;

        // 7. Normal / Inverse Display
        self.send_cmd(0xA6)?;
        
        // 8. Multiplex Ratio
        self.send_cmds(&[0xA8, 0x7F])?;

        // 9. Display Offset
        self.send_cmds(&[0xD3, 0x60])?; 

        // 10. Display Clock Divide Ratio / Oscillator Frequency
        self.send_cmds(&[0xD5, 0x51])?; 

        // 11. COM Output Scan Direction (通常表示)
        self.send_cmd(0xC0)?; // ★ ここを修正 ★

        // 12. Pre-charge Period
        self.send_cmds(&[0xD9, 0x22])?;
        
        // 13. COM Pins Hardware Configuration
        self.send_cmds(&[0xDA, 0x12])?;

        // 14. VCOMH Deselect Level
        self.send_cmds(&[0xDB, 0x35])?;

        // 15. Charge Pump
        self.send_cmds(&[0xAD, 0x8B])?;
        
        // 16. ディスプレイをオンにする
        self.send_cmd(0xAF)?;

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
            // ページアドレス、カラムアドレスを設定
            self.send_cmd(0xB0 + page as u8)?;
            self.send_cmd(0x00)?;
            self.send_cmd(0x10)?;

            // 各ページ128バイトのデータを送信
            let start_index = page * page_width;
            let end_index = start_index + page_width;
            let page_data = &self.buffer[start_index..end_index];

            // データの送信
            // heapless::Vec を使ってコントロールバイトとデータを結合
            let mut data_payload = heapless::Vec::<u8, {1 + 128}>::new();
            data_payload.push(0x40).unwrap(); // コントロールバイトを先頭にプッシュ
            data_payload.extend_from_slice(page_data).unwrap(); // ページデータを追加

            self.i2c.write(self.address, &data_payload)?;
        }
        Ok(())
    }
}