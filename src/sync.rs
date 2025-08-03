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
impl<I2C, E> Sh1107g<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Display initialize sequence
    pub fn init(&mut self) -> Result<(), E> {
        self.send_cmd(DISPLAY_OFF)?; // Display Off
        self.send_cmdandarg(CLOCK_DIVIDE_CMD, CLOCK_DIVIDE_DATA)?; // Set Display Clock Divide Ratio / Osc Frequency (Pythonで0x51)
        self.send_cmdandarg(SET_MULTIPLEX_RATIO, MULTIPLEX_RATIO_DATA)?; // Set Multiplex Ratio (128行対応)
        self.send_cmdandarg(DISPLAY_OFFSET_CMD, DISPLAY_OFFSET_DATA)?; // Set Display Offset (Pythonで0x60)
        self.send_cmdandarg(CHARGE_PUMP_ON_CMD, CHARGE_PUMP_ON_DATA)?; // Set Charge Pump (Pythonで0x8B, データシートでは8BhがEnable)
        self.send_cmdandarg(0xDA, 0x12)?; // Set COM Pins Hardware Config (Pythonで0x12)
        self.send_cmd(PAGE_ADDRESSING_CMD)?; // Set Memory Addressing Mode (Page Addressing Mode)
        self.send_cmd(CONTRAST_CONTROL_CMD)?; // Set Contrast Control
        self.send_cmdandarg(CONTRAST_CONTROL_CMD, CONTRAST_CONTROL_DATA)?; // Contrast Control (0x2Fは一般的な値)
        self.send_cmd(SEGMENT_REMAP)?; // Set Segment Remap (通常はA0hかA1h)
        self.send_cmd(COM_OUTPUT_SCAN_DIR)?; // Set COM Output Scan Direction (C0h: Normal, C8h: Re-mapped)
        self.send_cmdandarg(PRECHARGE_CMD, PRECHARGE_DATA)?; // Set Pre-charge Period
        self.send_cmdandarg(VCOM_DESELECT_CMD, VCOM_DESELECT_DATA)?; // Set VCOM Deselect Level
        self.send_cmd(0xA4)?; // Set Entire Display ON / OFF (A4h: Normal Display)
        self.send_cmd(0xA6)?; // Set Normal / Inverse Display (A6h: Normal)
        self.send_cmd(DISPLAY_ON)?; // Display ON

        Ok(())
    }

    /// IIC write command byte
    fn send_cmd(&mut self, cmd: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd])
    }

    /// IIC write command byte and data byte
    fn send_cmdandarg(&mut self, cmd: u8, arg: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd, arg])
    }

    /// Rendering
    // Send self internal buffer
    pub fn flush(&mut self) -> Result<(), E> {
        // SH1107G is page addressing mode and 128 byte/page
        // 128/8 = 16 page because 128x128 pixels
        for page in 0..16 { // 0 to 15
            self.send_cmd(0xB0 + page)?; // Set Page Address (B0h ~ BFh)
            self.send_cmd(0x00)?; // Set Lower Column Address (0x00)
            self.send_cmd(0x10)?; // Set Higher Column Address (0x10)

            // `buffer` は2048バイト全体で、各ページ128バイトなので
            // buffer[page * 128 .. (page + 1) * 128] で該当ページのスライスを取得

            // Cast to usize and perform calculation
            let page_usize = page as usize;
            let width_usize = DISPLAY_WIDTH as usize;

            let start_index = page_usize * (width_usize / 8);
            let end_index = (page_usize + 1) * (width_usize / 8);

            // スライスもusizeの範囲で指定
            // 内部バッファ保持
            let page_data = &self.buffer[start_index..end_index];

            // I2Cのwriteは1回の呼び出しで送信できるデータ量に制限がある場合があるため、
            // 16バイトずつ分割して送信するロジックは理にかなっている。
            // 各ページのデータを16バイトチャンクで送信
            for chunk in page_data.chunks(16) {
                let mut buf: heapless::Vec<u8, 17> = heapless::Vec::new(); // 制御バイト1 + データ最大16バイト
                buf.push(0x40).unwrap(); // control byte for data (0x40)
                buf.extend_from_slice(chunk).unwrap();
                self.i2c.write(self.address, &buf)?;
            }
        }
        Ok(())
    }
}
