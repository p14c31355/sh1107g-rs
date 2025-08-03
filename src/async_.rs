#[cfg(feature = "async_")]
use crate::{cmds::*, BuilderError, Sh1107g, Sh1107gBuilder, DISPLAY_WIDTH};

#[cfg(feature = "async_")]
use core::result::Result;
#[cfg(feature = "async_")]
use core::result::Result::Ok;

#[cfg(feature = "async_")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    pub async fn build_async(self) -> Result<Sh1107g<I2C>, BuilderError> {
        let i2c = self.i2c.ok_or(BuilderError::NoI2cConnected)?;
        let oled = Sh1107g::new(i2c, self.address);
        Ok(oled)
    }
}

#[cfg(feature = "async_")]
// Sh1107g impl block
impl<I2C, E> Sh1107g<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    /// Init display
    pub async fn init_async(&mut self) -> Result<(), E> {
        // 正確な初期化シーケンスの例 (上記のPythonドライバのロジックとデータシートに基づき再構成)
        self.send_cmd_async(DISPLAY_OFF).await?; // Display Off
        self.send_cmdandarg_async(CLOCK_DIVIDE_CMD, CLOCK_DIVIDE_DATA).await?; // Set Display Clock Divide Ratio / Osc Frequency (Pythonで0x51)
        self.send_cmdandarg_async(SET_MULTIPLEX_RATIO, MULTIPLEX_RATIO_DATA).await?; // Set Multiplex Ratio (128行対応)
        self.send_cmdandarg_async(DISPLAY_OFFSET_CMD, DISPLAY_OFFSET_DATA).await?; // Set Display Offset (Pythonで0x60)
        self.send_cmdandarg_async(CHARGE_PUMP_ON_CMD, CHARGE_PUMP_ON_DATA).await?; // Set Charge Pump (Pythonで0x8B, データシートでは8BhがEnable)
        self.send_cmdandarg_async(0xDA, 0x12).await?; // Set COM Pins Hardware Config (Pythonで0x12)
        self.send_cmd_async(PAGE_ADDRESSING_CMD).await?; // Set Memory Addressing Mode (Page Addressing Mode)
        self.send_cmd_async(CONTRAST_CONTROL_CMD).await?; // Set Contrast Control
        self.send_cmdandarg_async(CONTRAST_CONTROL_CMD, CONTRAST_CONTROL_DATA).await?; // Contrast Control (0x2Fは一般的な値)
        self.send_cmd_async(SEGMENT_REMAP).await?; // Set Segment Remap (通常はA0hかA1h)
        self.send_cmd_async(COM_OUTPUT_SCAN_DIR).await?; // Set COM Output Scan Direction (C0h: Normal, C8h: Re-mapped)
        self.send_cmdandarg_async(PRECHARGE_CMD, PRECHARGE_DATA).await?; // Set Pre-charge Period
        self.send_cmdandarg_async(VCOM_DESELECT_CMD, VCOM_DESELECT_DATA).await?; // Set VCOM Deselect Level
        self.send_cmd_async(0xA4).await?; // Set Entire Display ON / OFF (A4h: Normal Display)
        self.send_cmd_async(0xA6).await?; // Set Normal / Inverse Display (A6h: Normal)
        self.send_cmd_async(DISPLAY_ON).await?; // Display ON

        Ok(())
    }

    /// 単一コマンドを送信
    async fn send_cmd_async(&mut self, cmd: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd]).await
    }

    /// コマンドと引数を送信
    async fn send_cmdandarg_async(&mut self, cmd: u8, arg: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd, arg]).await
    }

    /// Rendering
    // Send self internal buffer
    pub async fn flush_async(&mut self) -> Result<(), E> {
        // SH1107Gはページアドレッシングモードで、各ページ128バイト
        // 128x128ピクセルなので、128/8 = 16ページ
        for page in 0..16 { // 0から15ページまで
            self.send_cmd_async(0xB0 + page).await?; // Set Page Address (B0h ~ BFh)
            self.send_cmd_async(0x00).await?; // Set Lower Column Address (0x00)
            self.send_cmd_async(0x10).await?; // Set Higher Column Address (0x10)

            // 各ページ128バイトのデータを送信
            // `buffer` は2048バイト全体で、各ページ128バイトなので
            // buffer[page * 128 .. (page + 1) * 128] で該当ページのスライスを取得

            // page も usize にキャストして演算
            let page_usize = page as usize; // <-- ここでusizeにキャスト
            let width_usize = DISPLAY_WIDTH as usize; // <-- ここもusizeにキャスト

            // インデックス計算をusizeで行う
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
                self.i2c.write(self.address, &buf).await?;
            }
        }
        Ok(())
    }
}