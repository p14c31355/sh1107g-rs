// cmds.rs
pub const DISPLAY_OFF: &[u8] = &[0xAE];
pub const DISPLAY_ON: &[u8]  = &[0xAF];
pub const ENTIRE_DISPLAY_RESUME: &[u8] = &[0xA4];
pub const INVERT_NORMAL: &[u8] = &[0xA6];
pub const SEGMENT_REMAP_REMAPPED: &[u8] = &[0xA1];
pub const COM_SCAN_NORMAL: &[u8] = &[0xC0];

pub fn contrast(level: u8) -> [u8; 2] {
    [0x81, level]
}

pub fn set_start_line(line: u8) -> [u8; 2] {
    [0xDC, line & 0x7F]
}

pub fn display_offset(offset: u8) -> [u8; 2] {
    [0xD3, offset]
}
