use embedded_hal::i2c::I2c;
use heapless::Vec;

/// SH1107G I2C OLEDドライバ
// sh1107g-driver/src/lib.rs

// 既存のSh1107g構造体はそのまま残す
pub struct Sh1107g<I2C> {
    i2c: I2C,
    address: u8,
    // 必要に応じて、DisplayRotationやDisplaySizeなどの設定をここに保持する
    // 今回はBuilderで設定し、最終的なSh1107gに渡す形にするため、直接は持たせない
}

// Builder構造体
pub struct Sh1107gBuilder<I2C> {
    i2c: Option<I2C>, // I2CインスタンスはOptionで、後から設定される
    address: u8,      // デフォルトアドレスを設定しておくか、Optionにする
    // ここに、初期化に必要な他の設定値（例: サイズ、回転など）を追加
    // size: Option<DisplaySize>, // DisplaySize構造体が定義されていれば
    // rotation: DisplayRotation, // デフォルト値を持たせるかOptionにする
}

// （仮）DisplaySizeとDisplayRotationの定義
// これらはembedded-graphicsクレートから提供されることが多いですが、
// まずは仮で定義しておきます。
// 後でembedded-graphicsを導入するときに置き換えます。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplaySize {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayRotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}
// デフォルト値
impl Default for DisplayRotation {
    fn default() -> Self {
        DisplayRotation::Rotate0
    }
}

// Sh1107gBuilder の impl ブロック
impl<I2C> Sh1107gBuilder<I2C> {
    /// 新しいBuilderインスタンスを作成する。
    /// デフォルトのI2Cアドレスを指定する。
    pub fn new() -> Self {
        Self {
            i2c: None,
            address: 0x3C, // デフォルトI2Cアドレス (0x3Cは一般的)
            // size: None,
            // rotation: DisplayRotation::default(),
        }
    }

    /// I2Cインターフェースを接続する。
    pub fn connect_i2c(mut self, i2c: I2C) -> Self {
        self.i2c = Some(i2c);
        self
    }

    /// I2Cアドレスを設定する。
    pub fn with_address(mut self, address: u8) -> Self {
        self.address = address;
        self
    }

    // 必要に応じて他の設定メソッドを追加
    // 例えば、ディスプレイサイズを設定するメソッド
    // pub fn with_size(mut self, size: DisplaySize) -> Self {
    //     self.size = Some(size);
    //     self
    // }

    // ディスプレイの回転を設定するメソッド
    // pub fn with_rotation(mut self, rotation: DisplayRotation) -> Self {
    //     self.rotation = rotation;
    //     self
    // }
}

// Sh1107gBuilder の impl ブロック内 (続き)

impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// 設定に基づきSh1107gインスタンスを構築する。
    pub fn build(self) -> Result<Sh1107g<I2C>, BuilderError> {
        let i2c = self.i2c.ok_or(BuilderError::NoI2cConnected)?;
        // let size = self.size.ok_or(BuilderError::NoDisplaySizeDefined)?; // サイズが必須の場合

        let mut oled = Sh1107g {
            i2c,
            address: self.address,
            // size: size,
            // rotation: self.rotation,
        };

        // ここでディスプレイの初期化を自動的に行っても良いし、
        // build() はインスタンスの作成のみに責任を持ち、init() は別途呼び出すようにしても良い。
        // 今回はシンプルにインスタンス作成まで。
        Ok(oled)
    }
}

// Builderパターンで発生しうるエラーを定義
#[derive(Debug)]
pub enum BuilderError {
    NoI2cConnected,
    // NoDisplaySizeDefined, // サイズが必須の場合
}
// embedded-halのErrorトレイトにも対応させる必要があるかもしれません
// impl embedded_hal::i2c::Error for BuilderError { ... }
// impl From<BuilderError> for YourDriverError { ... } など

// Sh1107g の impl ブロック
impl<I2C, E> Sh1107g<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// 新しいドライバインスタンスを作成
    fn new(i2c: I2C, address: u8) -> Self {
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