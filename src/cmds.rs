/// cmds.rs

#[cfg(feature = "debug_log")]
use dvcdbg::logger::Logger;

pub const DISPLAY_OFF: u8 = 0xAE;
pub const CHARGE_PUMP_ON_CMD: u8 = 0xAD;
pub const CHARGE_PUMP_ON_DATA: u8 = 0x8B;
pub const SET_MULTIPLEX_RATIO: u8 = 0xA8;
pub const MULTIPLEX_RATIO_DATA: u8 = 0x7F;
pub const PAGE_ADDRESSING_CMD: u8 = 0x20;
pub const SEGMENT_REMAP: u8 = 0xA0;
pub const COM_OUTPUT_SCAN_DIR: u8 = 0xC0;
pub const DISPLAY_START_LINE_CMD: u8 = 0xDC;
pub const DISPLAY_START_LINE_DATA: u8 = 0x00;
pub const CONTRAST_CONTROL_CMD: u8 = 0x81;
pub const CONTRAST_CONTROL_DATA: u8 = 0x2F;
pub const DISPLAY_OFFSET_CMD: u8 = 0xD3;
pub const DISPLAY_OFFSET_DATA: u8 = 0x60;
pub const PRECHARGE_CMD: u8 = 0xD9;
pub const PRECHARGE_DATA: u8 = 0x22;
pub const VCOM_DESELECT_CMD: u8 = 0xDB;
pub const VCOM_DESELECT_DATA: u8 = 0x35;
pub const CLOCK_DIVIDE_CMD: u8 = 0xD5;
pub const CLOCK_DIVIDE_DATA: u8 = 0x51;
pub const SET_COM_PINS_CMD: u8 = 0xDA;
pub const SET_COM_PINS_DATA: u8 = 0x12;
pub const SET_ENTIRE_DISPLAY_ON_OFF_CMD: u8 = 0xA4;
pub const SET_NORMAL_INVERSE_DISPLAY_CMD: u8 = 0xA6;
pub const DISPLAY_ON: u8 = 0xAF;

#[cfg(feature = "debug_log")]
pub fn log_init_sequence<L: Logger>(logger: &mut L) {
    let cmds: &[u8] = &[
        DISPLAY_OFF,
        CHARGE_PUMP_ON_CMD,
        CHARGE_PUMP_ON_DATA,
        SET_MULTIPLEX_RATIO,
        MULTIPLEX_RATIO_DATA,
        PAGE_ADDRESSING_CMD,
        SEGMENT_REMAP,
        COM_OUTPUT_SCAN_DIR,
        DISPLAY_START_LINE_CMD,
        DISPLAY_START_LINE_DATA,
        CONTRAST_CONTROL_CMD,
        CONTRAST_CONTROL_DATA,
        DISPLAY_OFFSET_CMD,
        DISPLAY_OFFSET_DATA,
        PRECHARGE_CMD,
        PRECHARGE_DATA,
        VCOM_DESELECT_CMD,
        VCOM_DESELECT_DATA,
        CLOCK_DIVIDE_CMD,
        CLOCK_DIVIDE_DATA,
        SET_COM_PINS_CMD,
        SET_COM_PINS_DATA,
        SET_ENTIRE_DISPLAY_ON_OFF_CMD,
        SET_NORMAL_INVERSE_DISPLAY_CMD,
        DISPLAY_ON,
    ];

    logger.log_bytes("init_sequence", cmds);
}