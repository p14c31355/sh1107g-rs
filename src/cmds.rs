//! SH1107G Command Definitions
//!
//! 型安全なコマンド構造体で表現。
//! 単純なコマンドは `const`、可変引数付きは `fn` で生成。
//!
//! 参考: Seeed Studio SH1107G Datasheet V2.1

/// 表示ON/OFF制御
pub enum DisplayPower {
    Off = 0xAE,
    On  = 0xAF,
}

/// コントラスト設定
pub struct Contrast(pub u8);
impl Contrast {
    pub fn to_bytes(&self) -> [u8; 2] {
        [0x81, self.0]
    }
}

/// 全表示制御
pub enum EntireDisplay {
    Resume = 0xA4,
    ForceOn = 0xA5,
}

/// 表示反転制御
pub enum Invert {
    Normal = 0xA6,
    Inverted = 0xA7,
}

/// アドレスモード設定
pub enum AddressMode {
    Page = 0x20,
    Horizontal = 0x21,
}

/// ページアドレス設定（0～15）
pub struct SetPageAddress(pub u8);
impl SetPageAddress {
    pub fn to_bytes(&self) -> [u8; 1] {
        [0xB0 | (self.0 & 0x0F)]
    }
}

/// カラムアドレス設定（0～127）
pub struct SetColumnAddress(pub u8);
impl SetColumnAddress {
    pub fn to_bytes(&self) -> [u8; 2] {
        [
            0x10 | ((self.0 >> 4) & 0x0F),
            self.0 & 0x0F
        ]
    }
}

/// 表示開始ライン設定（0～127）
pub struct SetStartLine(pub u8);
impl SetStartLine {
    pub fn to_bytes(&self) -> [u8; 1] {
        [0xDC | (self.0 & 0x7F)]
    }
}

/// 内部クロック設定
pub struct SetClockDiv {
    pub divide_ratio: u8,
    pub oscillator_freq: u8,
}
impl SetClockDiv {
    pub fn to_bytes(&self) -> [u8; 2] {
        [
            0xD5,
            ((self.oscillator_freq & 0x0F) << 4) | (self.divide_ratio & 0x0F),
        ]
    }
}

/// チャージポンプ制御
pub struct ChargePump(pub bool);
impl ChargePump {
    pub fn to_bytes(&self) -> [u8; 2] {
        [0xAD, if self.0 { 0x8B } else { 0x8A }]
    }
}

/// マルチプレックス比設定
pub struct MultiplexRatio(pub u8);
impl MultiplexRatio {
    pub fn to_bytes(&self) -> [u8; 2] {
        [0xA8, self.0]
    }
}

/// COM出力スキャン方向
pub enum ComOutputScanDirection {
    Normal = 0xC0,
    Remapped = 0xC8,
}

/// COMピン設定
pub struct SetComPins(pub u8);
impl SetComPins {
    pub fn to_bytes(&self) -> [u8; 2] {
        [0xDA, self.0]
    }
}

/// プリチャージ期間設定
pub struct PreChargePeriod(pub u8);
impl PreChargePeriod {
    pub fn to_bytes(&self) -> [u8; 2] {
        [0xD9, self.0]
    }
}

/// VCOMHデセレクトレベル設定
pub struct VcomhDeselectLevel(pub u8);
impl VcomhDeselectLevel {
    pub fn to_bytes(&self) -> [u8; 2] {
        [0xDB, self.0]
    }
}

/// セグメントリマップ
pub enum SegmentRemap {
    Normal = 0xA0,
    Remap = 0xA1,
}

/// 表示オフセット設定
pub struct SetDisplayOffset(pub u8);
impl SetDisplayOffset {
    pub fn to_bytes(&self) -> [u8; 2] {
        [0xD3, self.0]
    }
}

/// 汎用: 1バイトコマンド（引数なし）
pub const fn cmd(byte: u8) -> [u8; 1] {
    [byte]
}

/*
pub const SH1107G_INIT_CMDS: &[u8] = &[
    0xAE, // Display OFF
    0xDC, 0x00, // Display start line = 0
    0x81, 0x2F, // Contrast
    0x20, // Memory addressing mode: page
    0xA0, // Segment remap normal
    0xC0, // Common output scan direction normal
    0xA4, // Entire display ON from RAM
    0xA6, // Normal display
    0xA8, 0x7F, // Multiplex ratio 128
    0xD3, 0x60, // Display offset
    0xD5, 0x51, // Oscillator frequency
    0xD9, 0x22, // Pre-charge period
    0xDB, 0x35, // VCOM deselect level
    0xAD, 0x8A, // DC-DC control
    0xAF,       // Display ON
];
*/