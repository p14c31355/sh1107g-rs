//! Display variant

/// Trait to represent a speciffic display
pub trait DisplayVariant {
    /// Width of display
    const WIDTH: u8;
    /// Height of display
    const HEIGHT: u8;
    /// Coumn offset
    const COLUMN_OFFSET: u8 = 0;
    /// Large Page Address
    const LARGE_PAGE_ADDRESS: bool = false;

    /// Get integral dimensions from DisplaySize
    fn dimensions() -> (u8, u8) {
        (Self::WIDTH, Self::HEIGHT)
    }
}

//! Display rotation

/// Display rotation
#[derive(Clone, Copy)]
pub enum DisplayRotation {
    /// No rotation, normal display
    Rotate0,
    /// Rotate by 90 degress clockwise
    Rotate90,
    /// Rotate by 180 degress clockwise
    Rotate180,
    /// Rotate 270 degress clockwise
    Rotate270,
}