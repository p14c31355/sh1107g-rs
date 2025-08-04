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
    // ... (既存のnew関数など)

    // コマンドを単独で送信するヘルパー関数
    fn send_cmd(&mut self, cmd: u8) -> Result<(), E> {
        let payload = [0x80, cmd]; // コントロールバイト0x80を付加
        self.i2c.write(self.address, &payload)
    }

    // コマンドと引数をセットで送信するヘルパー関数
    fn send_cmdandarg(&mut self, cmd: u8, arg: u8) -> Result<(), E> {
        let payload = [0x80, cmd, 0x80, arg]; // コントロールバイト0x80を各コマンドの前に付加
        self.i2c.write(self.address, &payload)
    }

    /// Init display
    pub fn init(&mut self) -> Result<(), E> {
        self.send_cmd(0xAE)?;        // Display OFF
        self.send_cmdandarg(0xD5, 0x51)?; // Set Display Clock Div
        self.send_cmdandarg(0xA8, 0x7F)?; // Set Multiplex Ratio
        self.send_cmdandarg(0xD3, 0x60)?; // Display Offset
        self.send_cmd(0x40)?;        // Display Start Line
        self.send_cmdandarg(0xAD, 0x8B)?; // Charge Pump On
        self.send_cmd(0xA1)?;        // Segment Remap (re-mapped)
        self.send_cmd(0xC8)?;        // COM Output Scan Direction (re-mapped)
        self.send_cmdandarg(0xDA, 0x12)?; // COM Pins Hardware Configuration
        self.send_cmdandarg(0x81, 0x2F)?; // Contrast Control
        self.send_cmdandarg(0xD9, 0x22)?; // Pre-charge Period
        self.send_cmdandarg(0xDB, 0x35)?; // VCOMH Deselect Level
        self.send_cmd(0xA4)?;        // Entire Display On
        self.send_cmd(0xA6)?;        // Normal Display
        self.send_cmdandarg(0x20, 0x02)?; // Memory Addressing Mode (Page)
        self.send_cmd(0xAF)?;        // Display ON

        Ok(())
    }

    /// Rendering
    // Send self internal buffer
    pub fn flush(&mut self) -> Result<(), E> {
        use crate::DISPLAY_HEIGHT;

        let page_count = DISPLAY_HEIGHT as usize / 8;
        let page_width = DISPLAY_WIDTH as usize;

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
            // SH1107Gは1回のI2Cトランザクションで複数のデータを送る際に、
            // コントロールバイト`0x40`を先頭に1回だけ付加する。
            let mut data_payload = heapless::Vec::<u8, {1 + 128}>::new(); // 1 (control byte) + 128 (page_width)
            data_payload.push(0x40).unwrap();
            data_payload[1..].copy_from_slice(page_data);
            self.i2c.write(self.address, &data_payload)?;
        }

        Ok(())
    }
}