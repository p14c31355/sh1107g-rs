pub const DISPLAY_OFF: &[u8] = &[0xAE];
pub const DISPLAY_ON: &[u8]  = &[0xAF];

pub const CONTRAST: &[u8; 2] = &[0x81, 0x2F];

pub const ENTIRE_DISPLAY_RESUME: &[u8] = &[0xA4];
pub const ENTIRE_DISPLAY_FORCE: &[u8]  = &[0xA5];

pub const INVERT_NORMAL: &[u8] = &[0xA6];
pub const INVERT_INVERTED: &[u8] = &[0xA7];

pub const SEGMENT_REMAP_NORMAL: &[u8] = &[0xA0];
pub const SEGMENT_REMAP_REMAP: &[u8] = &[0xA1];

pub const COM_SCAN_NORMAL: &[u8] = &[0xC0];
pub const COM_SCAN_REMAP: &[u8] = &[0xC8];

pub fn set_start_line(line: u8) -> [u8; 2] {
    [0xDC, line & 0x7F]
}

pub fn multiplex_ratio(ratio: u8) -> [u8; 2] {
    [0xA8, ratio]
}

pub fn display_offset(offset: u8) -> [u8; 2] {
    [0xD3, offset]
}

pub fn clock_div(divide_ratio: u8, osc_freq: u8) -> [u8; 2] {
    [0xD5, ((osc_freq & 0x0F) << 4) | (divide_ratio & 0x0F)]
}

pub fn precharge_period(period: u8) -> [u8; 2] {
    [0xD9, period]
}

pub fn vcomh_deselect(level: u8) -> [u8; 2] {
    [0xDB, level]
}

pub fn charge_pump(enable: bool) -> [u8; 2] {
    [0xAD, if enable { 0x8B } else { 0x8A }]
}
