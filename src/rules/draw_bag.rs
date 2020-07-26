//! The draw pack from which we can draw tiles

use crate::rules::color_pack::*;
use crate::rules::Tile;

// Quick type aliasing, the draw bag and the discard pile are actually the same type
pub type DrawBag = ColorPack;

impl ColorPack {
    /// Creates a new draw bag when a new game start
    pub fn init_draw_bag() -> Self {
        Self::init_sized_bag(20)
    }
}

// An iterator over tiles in the bag
impl Iterator for DrawBag {
    type Item = Tile;
    fn next(&mut self) -> Option<Self::Item> {
        // If the bag is empty we cannot draw
        if self.is_empty() {
            None
        } else {
            // Otherwise we uniformly draw a tile in the bag
            let mut tile_number = rand::random::<u8>() % self.size() as u8;

            // Find color of the tile and remove it
            let color = self
                .tiles()
                .iter()
                .position(move |&s| {
                    if s > tile_number {
                        true
                    } else {
                        tile_number -= s;
                        false
                    }
                })
                .expect("Error finding color when drawing tile") as u8;

            self.remove_single_tile(color);

            Some(color)
        }
    }
}
