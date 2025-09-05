// cmds.rs
pub const SH1107G_INIT_CMDS: &[&[u8]] = &[
    &[0xAE],       // Display OFF
    &[0xD5, 0x51], // Display clock divide/oscillator freq
    &[0xA8, 0x7F], // Multiplex ratio 128 (for SH1107G full 128 rows)
    &[0xD3, 0x60], // Display offset
    &[0x40],       // Display start line = 0
    &[0xA1],       // Segment remap (mirror horizontally, often needed)
    &[0xC8],       // COM scan direction remapped (vertical flip)
    &[0x81, 0x2F], // Contrast
    &[0xAD, 0x8B], // DC-DC control (internal regulator)
    &[0xD9, 0x22], // Pre-charge period
    &[0xDB, 0x35], // VCOM deselect level
    &[0xA4],       // Entire display ON from RAM
    &[0xA6],       // Normal display
    &[0xAF],       // Display ON
];
