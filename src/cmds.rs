pub enum DisplayPower { Off = 0xAE, On = 0xAF }

pub struct Contrast(pub u8);
impl Contrast {
    pub fn to_bytes(&self) -> [u8; 2] { [0x81, self.0] }
}

pub enum EntireDisplay { Resume = 0xA4, ForceOn = 0xA5 }
pub enum Invert { Normal = 0xA6, Inverted = 0xA7 }
pub enum SegmentRemap { Normal = 0xA0, Remap = 0xA1 }
pub enum ComOutputScanDirection { Normal = 0xC0, Remapped = 0xC8 }
pub enum AddressMode { Page = 0x20, Horizontal = 0x21 }

pub struct SetStartLine(pub u8);
impl SetStartLine { pub fn to_bytes(&self) -> [u8; 1] { [0xDC | (self.0 & 0x7F)] } }

pub struct MultiplexRatio(pub u8);
impl MultiplexRatio { pub fn to_bytes(&self) -> [u8; 2] { [0xA8, self.0] } }

pub struct SetDisplayOffset(pub u8);
impl SetDisplayOffset { pub fn to_bytes(&self) -> [u8; 2] { [0xD3, self.0] } }

pub struct SetClockDiv { pub divide_ratio: u8, pub oscillator_freq: u8 }
impl SetClockDiv {
    pub fn to_bytes(&self) -> [u8; 2] { [0xD5, ((self.oscillator_freq & 0x0F) << 4) | (self.divide_ratio & 0x0F)] }
}

pub struct ChargePump(pub bool);
impl ChargePump { pub fn to_bytes(&self) -> [u8; 2] { [0xAD, if self.0 { 0x8B } else { 0x8A }] } }

pub struct PreChargePeriod(pub u8);
impl PreChargePeriod { pub fn to_bytes(&self) -> [u8; 2] { [0xD9, self.0] } }

pub struct VcomhDeselectLevel(pub u8);
impl VcomhDeselectLevel { pub fn to_bytes(&self) -> [u8; 2] { [0xDB, self.0] } }

pub struct SetComPins(pub u8);
impl SetComPins { pub fn to_bytes(&self) -> [u8; 2] { [0xDA, self.0] } }

pub struct SetPageAddress(pub u8);
impl SetPageAddress { pub fn to_bytes(&self) -> [u8; 1] { [0xB0 | (self.0 & 0x0F)] } }

pub struct SetColumnAddress(pub u8);
impl SetColumnAddress { pub fn to_bytes(&self) -> [u8; 2] { [0x10 | ((self.0 >> 4) & 0x0F), self.0 & 0x0F] } }

pub const fn cmd(byte: u8) -> [u8; 1] { [byte] }
