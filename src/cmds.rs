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

pub const fn cmd(byte: u8) -> [u8; 1] {
    [byte]
}

/*
pub const SH1107G_INIT_CMDS: &[u8] = &[
    0xAE, // Display OFF
    0xDC, 0x00, // Display start line = 0
    0x81, 0x2F, // Contrast
    0x20,  0x02, // Memory addressing mode: page
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