/* 
  PythonドライバとSH1107データシートから導出されたコマンド列
  コマンドを2バイトずつ（コマンドバイト + データバイト）送信
  ただし、0xAE, 0x20, 0xA0, 0xC0, 0xAF は単独コマンド
  そのため、cmds.chunks(2) の処理は注意が必要。
  個々のコマンドをsend_commandで送信するのがより確実。
*/

pub const DISPLAY_OFF: u8 = 0xAE;
pub const DISPLAY_ON: u8 = 0xAF;
pub const SET_MULTIPLEX_RATIO: u8 = 0xA8;
pub const MULTIPLEX_RATIO_DATA: u8 = 0x7F;
pub const CHARGE_PUMP_ON_CMD: u8 = 0xAD;
pub const CHARGE_PUMP_ON_DATA: u8 = 0x14;
pub const PAGE_ADDRESSING_CMD: u8 = 0x20;
pub const SEGMENT_REMAP: u8 = 0xA0;
pub const COM_OUTPUT_SCAN_DIR: u8 = 0xC0;
pub const DISPLAY_START_LINE_CMD: u8 = 0xDC;
pub const DISPLAY_START_LINE_DATA: u8 = 0x00;
pub const CONTRAST_CONTROL_CMD: u8 = 0x81;
pub const CONTRAST_CONTROL_DATA: u8 = 0xFF;
pub const DISPLAY_OFFSET_CMD: u8 = 0xD3;
pub const DISPLAY_OFFSET_DATA: u8 = 0x00;
pub const PRECHARGE_CMD: u8 = 0xD9;
pub const PRECHARGE_DATA: u8 = 0x22;
pub const VCOM_DESELECT_CMD: u8 = 0xDB;
pub const VCOM_DESELECT_DATA: u8 = 0x35;
pub const CLOCK_DIVIDE_CMD: u8 = 0xD5;
pub const CLOCK_DIVIDE_DATA: u8 = 0x51;