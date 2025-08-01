use embedded_hal::i2c::I2c;

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

        // ディスプレイの初期化を自動的に行っても良いし、build() はインスタンスの作成のみに責任を持ち、
        // init() は別途呼び出すようにしても良い。今回はシンプルにインスタンス作成まで。
        Ok(oled)
    }
}

// Sh1107g impl block
impl<I2C, E> Sh1107g<I2C>
where
    I2C: I2c<Error = E>,
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
    pub fn init(&mut self) -> Result<(), E> {
        // 正確な初期化シーケンスの例 (上記のPythonドライバのロジックとデータシートに基づき再構成)
        self.send_command_single(0xAE)?; // Display Off
        self.send_command_with_arg(0xD5, 0x51)?; // Set Display Clock Divide Ratio / Osc Frequency (Pythonで0x51)
        self.send_command_with_arg(0xA8, 0x7F)?; // Set Multiplex Ratio (128行対応)
        self.send_command_with_arg(0xD3, 0x60)?; // Set Display Offset (Pythonで0x60)
        self.send_command_with_arg(0xAD, 0x8B)?; // Set Charge Pump (Pythonで0x8B, データシートでは8BhがEnable)
        self.send_command_with_arg(0xDA, 0x12)?; // Set COM Pins Hardware Config (Pythonで0x12)
        self.send_command_single(0x20)?; // Set Memory Addressing Mode (Page Addressing Mode)
        self.send_command_single(0x81)?; // Set Contrast Control
        self.send_command_with_arg(0x81, 0x2F)?; // Contrast Control (0x2Fは一般的な値)
        self.send_command_single(0xA0)?; // Set Segment Remap (通常はA0hかA1h)
        self.send_command_single(0xC0)?; // Set COM Output Scan Direction (C0h: Normal, C8h: Re-mapped)
        self.send_command_with_arg(0xD9, 0x22)?; // Set Pre-charge Period
        self.send_command_with_arg(0xDB, 0x35)?; // Set VCOM Deselect Level
        self.send_command_single(0xA4)?; // Set Entire Display ON / OFF (A4h: Normal Display)
        self.send_command_single(0xA6)?; // Set Normal / Inverse Display (A6h: Normal)
        self.send_command_single(0xAF)?; // Display ON

        Ok(())
    }

    /// 単一コマンドを送信
    fn send_command_single(&mut self, cmd: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd])
    }

    /// コマンドと引数を送信
    fn send_command_with_arg(&mut self, cmd: u8, arg: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd, arg])
    }

    /// Rendering
    // Send self internal buffer
    pub fn flush(&mut self) -> Result<(), E> {
        // SH1107Gはページアドレッシングモードで、各ページ128バイト
        // 128x128ピクセルなので、128/8 = 16ページ
        for page in 0..16 { // 0から15ページまで
            self.send_command_single(0xB0 + page)?; // Set Page Address (B0h ~ BFh)
            self.send_command_single(0x00)?; // Set Lower Column Address (0x00)
            self.send_command_single(0x10)?; // Set Higher Column Address (0x10)

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
                self.i2c.write(self.address, &buf)?;
            }
        }
        Ok(())
    }

    /// 内部バッファをクリアする
    pub fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|b| *b = 0x00);
    }
}