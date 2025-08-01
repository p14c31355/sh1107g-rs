pub const CMDS: &[u8] = &[
  // 0xAE,       // Display OFF
  // 0xDC, 0x00, // Display start line = 0 (リセット後のデフォルト値)
  // 0x81, 0x2F, // Contrast control (コントラスト設定)
  // 0x20,       // Memory mode (Page addressing)
  // 0xA0,       // Segment remap (一般的な設定)
  // 0xC0,       // COM output scan dir (COMスキャン方向、通常はC0hかC8h)
  // 0xA8, 0x7F, // Multiplex ratio = 127 (128行の表示に対応)
  // 0xD3, 0x60, // Display offset = 0x60 (96ピクセルオフセット、128x128で重要)
  // 0xD5, 0x50, // Clock divide (クロック分周比と発振周波数)
  // 0xD9, 0x22, // Precharge (プリチャージ期間設定)
  // 0xDB, 0x35, // VCOM Deselect (VCOMHデセレクトレベル設定)
  // // 0xAD, 0x8A, // Charge pump on
  // 0xAF,       // Display ON
];

/* 
  PythonドライバとSH1107データシートから導出されたコマンド列
  コマンドを2バイトずつ（コマンドバイト + データバイト）送信
  ただし、0xAE, 0x20, 0xA0, 0xC0, 0xAF は単独コマンド
  そのため、cmds.chunks(2) の処理は注意が必要。
  個々のコマンドをsend_commandで送信するのがより確実。
  例: self.send_command(&[0xAE])?; self.send_command(&[0xDC, 0x00])?; ...
  提供されたコードの for chunk in cmds.chunks(2) は、コマンドとデータが常にペアであるという前提なので、
  実際のSH1107コマンド構造に合わせて変更が必要。
  例：send_command_single と send_command_with_arg に分けるなど
*/

pub const DISPLAY_OFF: u8 = 0xAE;
pub const DISPLAY_ON: u8 = 0xAF;
pub const SET_MULTIPLEX_RATIO: u8 = 0xA8;
pub const MULTIPLEX_RATIO_DATA: u8 = 0x7F;
pub const CHARGE_PUMP_ON_CMD: u8 = 0xAD;
pub const CHARGE_PUMP_ON_DATA: u8 = 0x8A;
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
pub const CLOCK_DIVIDE_DATA: u8 = 0x50;