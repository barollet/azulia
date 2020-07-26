pub type Tile = u8;
pub const BLUE: Tile = 0;
pub const CYAN: Tile = 1;
pub const RED: Tile = 2;
pub const BLACK: Tile = 3;
pub const ORANGE: Tile = 4;

pub const NB_COLORS: usize = 5;

mod color_pack;
mod discard_pile;
mod draw_bag;
mod fabric;
mod round_manager;

pub use discard_pile::*;
pub use draw_bag::*;
pub use fabric::*;
pub use round_manager::*;
