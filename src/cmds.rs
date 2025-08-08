/// cmds.rs
#[cfg(feature = "debug_log")]
use dvcdbg::logger::log_cmd;

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
/// デバッグ用の初期化コマンド列をログ出力する
pub fn log_init_sequence<L: Logger>(logger: &mut L) {
    log_cmd(logger, DISPLAY_OFF);
    log_cmd(logger, CHARGE_PUMP_ON_CMD);
    log_cmd(logger, CHARGE_PUMP_ON_DATA);
    log_cmd(logger, SET_MULTIPLEX_RATIO);
    log_cmd(logger, MULTIPLEX_RATIO_DATA);
    log_cmd(logger, PAGE_ADDRESSING_CMD);
    log_cmd(logger, SEGMENT_REMAP);
    log_cmd(logger, COM_OUTPUT_SCAN_DIR);
    log_cmd(logger, DISPLAY_START_LINE_CMD);
    log_cmd(logger, DISPLAY_START_LINE_DATA);
    log_cmd(logger, CONTRAST_CONTROL_CMD);
    log_cmd(logger, CONTRAST_CONTROL_DATA);
    log_cmd(logger, DISPLAY_OFFSET_CMD);
    log_cmd(logger, DISPLAY_OFFSET_DATA);
    log_cmd(logger, PRECHARGE_CMD);
    log_cmd(logger, PRECHARGE_DATA);
    log_cmd(logger, VCOM_DESELECT_CMD);
    log_cmd(logger, VCOM_DESELECT_DATA);
    log_cmd(logger, CLOCK_DIVIDE_CMD);
    log_cmd(logger, CLOCK_DIVIDE_DATA);
    log_cmd(logger, SET_COM_PINS_CMD);
    log_cmd(logger, SET_COM_PINS_DATA);
    log_cmd(logger, SET_ENTIRE_DISPLAY_ON_OFF_CMD);
    log_cmd(logger, SET_NORMAL_INVERSE_DISPLAY_CMD);
    log_cmd(logger, DISPLAY_ON);
}