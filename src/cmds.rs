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
        [0xAD, if self.0 { 0x8B } else { 0x10 }]
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

/// SH1107G の初期化コマンド列
pub const SH1107G_INIT_CMDS: &[u8] = {
    use DisplayPower::*;
    use AddressMode::*;
    use SegmentRemap::*;
    use ComOutputScanDirection::*;
    use EntireDisplay::*;
    use Invert::*;

    &[
        DisplayPower::Off.to_bytes()[0], // Display OFF
        SetStartLine(0x00).to_bytes()[0], // Display start line = 0
        Contrast(0x2F).to_bytes()[0], Contrast(0x2F).to_bytes()[1], // Contrast
        AddressMode::Page.to_bytes()[0], // Memory addressing mode: page
        SegmentRemap::Normal.to_bytes()[0], // Segment remap normal
        ComOutputScanDirection::Normal.to_bytes()[0], // Common output scan direction normal
        EntireDisplay::Resume.to_bytes()[0], // Entire display ON from RAM
        Invert::Normal.to_bytes()[0], // Normal display
        MultiplexRatio(0x7F).to_bytes()[0], MultiplexRatio(0x7F).to_bytes()[1], // Multiplex ratio 128
        SetDisplayOffset(0x60).to_bytes()[0], SetDisplayOffset(0x60).to_bytes()[1], // Display offset
        SetClockDiv { divide_ratio: 0x01, oscillator_freq: 0x05 }.to_bytes()[0], SetClockDiv { divide_ratio: 0x01, oscillator_freq: 0x05 }.to_bytes()[1], // Oscillator frequency
        PreChargePeriod(0x22).to_bytes()[0], PreChargePeriod(0x22).to_bytes()[1], // Pre-charge period
        VcomhDeselectLevel(0x35).to_bytes()[0], VcomhDeselectLevel(0x35).to_bytes()[1], // VCOM deselect level
        ChargePump(true).to_bytes()[0], ChargePump(true).to_bytes()[1], // DC-DC control
        DisplayPower::On.to_bytes()[0], // Display ON
    ]
};
