use lazy_static::lazy_static;
use naia_shared::{derive_serde, serde};

pub mod handle_input;

pub const HEXAGON_SIZE: f32 = 75.0;
pub const HEXAGON_HEIGHT: f32 = HEXAGON_SIZE * 2.0;
pub const HEXAGON_Y_SPACING: f32 = HEXAGON_HEIGHT * 0.75;
lazy_static! {
    pub static ref HEXAGON_WIDTH: f32 = f32::sqrt(3.0) * HEXAGON_SIZE;
    pub static ref HEXAGON_X_SPACING: f32 = *HEXAGON_WIDTH;
}

#[derive(Copy, Debug, Eq, Hash)]
#[derive_serde]
pub struct AxialCoordinates {
    pub column_q: u16,
    pub row_r: u16,
}

impl AxialCoordinates {
    pub fn new(q: u16, r: u16) -> AxialCoordinates {
        AxialCoordinates {
            column_q: q,
            row_r: r,
        }
    }
}
