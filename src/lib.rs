use embedded_hal::i2c::I2c;
use heapless::Vec;

/// SH1107G I2C OLEDドライバ
pub struct Sh1107g<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Sh1107g<I2C>
where
    I2C: I2c<Error = E>,
{
    /// 新しいドライバインスタンスを作成
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }

    /// ディスプレイの初期化シーケンスを実行
    pub fn init(&mut self) -> Result<(), E> {
        // PythonドライバとSH1107データシートから導出されたコマンド列
        let cmds: &[u8] = &[
            0xAE,       // Display OFF
            0xDC, 0x00, // Display start line = 0 (リセット後のデフォルト値)
            0x81, 0x2F, // Contrast control (コントラスト設定)
            0x20,       // Memory mode (Page addressing)
            0xA0,       // Segment remap (一般的な設定)
            0xC0,       // COM output scan dir (COMスキャン方向、通常はC0hかC8h)
            0xA8, 0x7F, // Multiplex ratio = 127 (128行の表示に対応)
            0xD3, 0x60, // Display offset = 0x60 (96ピクセルオフセット、128x128で重要)
            0xD5, 0x50, // Clock divide (クロック分周比と発振周波数)
            0xD9, 0x22, // Precharge (プリチャージ期間設定)
            0xDB, 0x35, // VCOM Deselect (VCOMHデセレクトレベル設定)
            0xAD, 0x8A, // Charge pump on (チャージポンプ有効化)
            0xAF,       // Display ON
        ];
        // コマンドを2バイトずつ（コマンドバイト + データバイト）送信
        // ただし、0xAE, 0x20, 0xA0, 0xC0, 0xAF は単独コマンド
        // そのため、cmds.chunks(2) の処理は注意が必要。
        // 個々のコマンドをsend_commandで送信するのがより確実。
        // 例: self.send_command(&[0xAE])?; self.send_command(&[0xDC, 0x00])?; ...
        // 提供されたコードの for chunk in cmds.chunks(2) は、コマンドとデータが常にペアであるという前提なので、
        // 実際のSH1107コマンド構造に合わせて変更が必要。
        // 例：send_command_single と send_command_with_arg に分けるなど

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

    /// 画面描画（バッファデータをディスプレイに書き込む）
    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), E> {
        // SH1107Gはページアドレッシングモードで、各ページ128バイト
        // 128x128ピクセルなので、128/8 = 16ページ
        for page in 0..16 { // 0から15ページまで
            self.send_command_single(0xB0 + page)?; // Set Page Address (B0h ~ BFh)
            self.send_command_single(0x00)?; // Set Lower Column Address (0x00)
            self.send_command_single(0x10)?; // Set Higher Column Address (0x10)

            // 各ページ128バイトのデータを送信
            // `buffer` は2048バイト全体で、各ページ128バイトなので
            // buffer[page * 128 .. (page + 1) * 128] で該当ページのスライスを取得
            let page_data = &buffer[(page * 128)..((page + 1) * 128)];

            // I2Cのwriteは1回の呼び出しで送信できるデータ量に制限がある場合があるため、
            // 16バイトずつ分割して送信するロジックは理にかなっている。
            for chunk in page_data.chunks(16) {
                let mut buf: Vec<u8, 17> = Vec::new(); // 制御バイト1 + データ最大16バイト
                buf.push(0x40).unwrap(); // control byte for data (0x40)
                buf.extend_from_slice(chunk).unwrap();
                self.i2c.write(self.address, &buf)?;
            }
        }
        Ok(())
    }
}