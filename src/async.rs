use embedded_hal_async::i2c::I2c; // async版のI2cトレイト

pub struct Sh1107g<I2C> {
    i2c: I2C,
    address: u8,
    buffer: [u8; BUFFER_SIZE], // Internal buffer
    // Configure in builder to Sh1107g struct
}

#[cfg(feature = "async")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    pub async fn build(self) -> Result<Sh1107g<I2C>, BuilderError> {
        let i2c = self.i2c.ok_or(BuilderError::NoI2cConnected)?;
        let oled = Sh1107g::new(i2c, self.address);
        Ok(oled)
    }
}

#[cfg(feature = "async")]
// Sh1107g impl block
impl<I2C, E> Sh1107g<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    // Make new driver instance & Degine function called by the builder
    // Initialise the internal buffer when called by builder
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
            buffer: [0x00; BUFFER_SIZE], // 全てオフで初期化
        }
    }

    /// Init display
    pub async fn init(&mut self) -> Result<(), E> {
        // 正確な初期化シーケンスの例 (上記のPythonドライバのロジックとデータシートに基づき再構成)
        self.send_command_single(0xAE).await?; // Display Off
        self.send_command_with_arg(0xD5, 0x51).await?; // Set Display Clock Divide Ratio / Osc Frequency (Pythonで0x51)
        self.send_command_with_arg(0xA8, 0x7F).await?; // Set Multiplex Ratio (128行対応)
        self.send_command_with_arg(0xD3, 0x60).await?; // Set Display Offset (Pythonで0x60)
        self.send_command_with_arg(0xAD, 0x8B).await?; // Set Charge Pump (Pythonで0x8B, データシートでは8BhがEnable)
        self.send_command_with_arg(0xDA, 0x12).await?; // Set COM Pins Hardware Config (Pythonで0x12)
        self.send_command_single(0x20).await?; // Set Memory Addressing Mode (Page Addressing Mode)
        self.send_command_single(0x81).await?; // Set Contrast Control
        self.send_command_with_arg(0x81, 0x2F).await?; // Contrast Control (0x2Fは一般的な値)
        self.send_command_single(0xA0).await?; // Set Segment Remap (通常はA0hかA1h)
        self.send_command_single(0xC0).await?; // Set COM Output Scan Direction (C0h: Normal, C8h: Re-mapped)
        self.send_command_with_arg(0xD9, 0x22).await?; // Set Pre-charge Period
        self.send_command_with_arg(0xDB, 0x35).await?; // Set VCOM Deselect Level
        self.send_command_single(0xA4).await?; // Set Entire Display ON / OFF (A4h: Normal Display)
        self.send_command_single(0xA6).await?; // Set Normal / Inverse Display (A6h: Normal)
        self.send_command_single(0xAF).await?; // Display ON

        Ok(())
    }

    /// 単一コマンドを送信
    async fn send_command_single(&mut self, cmd: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd]).await
    }

    /// コマンドと引数を送信
    async fn send_command_with_arg(&mut self, cmd: u8, arg: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd, arg]).await
    }

    /// Rendering
    // Send self internal buffer
    pub async fn flush(&mut self) -> Result<(), E> {
        // SH1107Gはページアドレッシングモードで、各ページ128バイト
        // 128x128ピクセルなので、128/8 = 16ページ
        for page in 0..16 { // 0から15ページまで
            self.send_command_single(0xB0 + page).await?; // Set Page Address (B0h ~ BFh)
            self.send_command_single(0x00).await?; // Set Lower Column Address (0x00)
            self.send_command_single(0x10).await?; // Set Higher Column Address (0x10)

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
                let mut buf: Vec<u8, 17> = Vec::new(); // 制御バイト1 + データ最大16バイト
                buf.push(0x40).unwrap(); // control byte for data (0x40)
                buf.extend_from_slice(chunk).unwrap();
                self.i2c.write(self.address, &buf).await?;
            }
        }
        Ok(())
    }

    /// 内部バッファをクリアする
    pub fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|b| *b = 0x00);
    }
}

#[cfg(feature = "async")]
impl<I2C, E> DrawTarget for Sh1107g<I2C>
where
    // I2Cトレイト境界は、DrawTarget自身はI2cトレイトを必要としないため、ここで指定する必要はない
    // むしろ、Sh1107gがI2CとEに依存していることを示すだけでよい
    Sh1107g<I2C>: Sized, // Self::Error が E であることを保証するため
    E: embedded_hal_async::i2c::Error + embedded_hal::i2c::Error, // 両方のエラー型に対応
{
    // DrawTarget define color dimension (monochro OLED = BinaryColor)
    type Color = BinaryColor;
    type Error = E; // embedded-halのI2Cエラーをそのまま使う

    /// ピクセルを描画する主要なメソッド
    fn draw_iter<PIXELS>(&mut self, pixels: PIXELS) -> Result<(), Self::Error>
    where
        PIXELS: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(Point { x, y }, color) in pixels {
            // 座標がディスプレイ範囲内かチェック
            if x < 0 || x >= DISPLAY_WIDTH as i32 || y < 0 || y >= DISPLAY_HEIGHT as i32 {
                continue; // 範囲外のピクセルはスキップ
            }

            // ピクセル座標からバッファのインデックスとビットマスクを計算
            // SH1107Gはページアドレッシングモードで、各バイトが縦8ピクセル
            let byte_index = (x as usize) + (y as usize / 8) * (DISPLAY_WIDTH as usize);
            let bit_mask = 1 << (y % 8); // バイト内のビット位置

            // バッファの範囲チェック（念のため）
            if byte_index >= BUFFER_SIZE {
                continue; // バッファ範囲外もスキップ
            }

            // 色に応じてバッファのビットをセットまたはクリア
            match color {
                BinaryColor::On => self.buffer[byte_index] |= bit_mask,  // ピクセルをON (セット)
                BinaryColor::Off => self.buffer[byte_index] &= !bit_mask, // ピクセルをOFF (クリア)
            }
        }
        Ok(())
    }

    /// Fill in with color
    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        let fill_byte = match color {
            BinaryColor::On => 0xFF,
            BinaryColor::Off => 0x00,
        };
        self.buffer.iter_mut().for_each(|b| *b = fill_byte);
        Ok(())
    }
}