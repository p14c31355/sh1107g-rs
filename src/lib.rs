pub mod common;
pub mod cmds;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "async")]
pub mod async_;

use heapless::Vec;


use crate::common::{DISPLAY_HEIGHT, DISPLAY_WIDTH, BUFFER_SIZE};
use crate::common::{Sh1107g, Sh1107gBuilder, BuilderError};
use crate::cmds::*;

/// SH1107G I2C OLED driver



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayRotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

// Default process
impl Default for DisplayRotation {
    fn default() -> Self {
        DisplayRotation::Rotate0
    }
}