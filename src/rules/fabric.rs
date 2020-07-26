use std::iter::FromIterator;

use crate::rules::color_pack::*;
use crate::rules::Tile;

pub type Fabric = ColorPack;

// Filling a fabric with an iterator of tiles
impl FromIterator<Tile> for Fabric {
    fn from_iter<I: IntoIterator<Item = Tile>>(iter: I) -> Self {
        let mut fabric: Fabric = Default::default();

        for tile in iter {
            fabric.add_single_tile(tile);
        }

        fabric
    }
}

impl Fabric {
    /// Returns a list of available colors in the fabric
    pub fn available_colors(&self) -> Vec<Tile> {
        self.tiles()
            .iter()
            .enumerate()
            .filter(|(_, &s)| s != 0)
            .map(|(i, _)| i as u8)
            .collect()
    }

    /// Remove all the tiles of the given color in the fabric and returns the number of removed
    /// tiles
    pub fn take(&mut self, color: u8) -> u8 {
        assert!(self.available_colors().contains(&(color as u8)));

        let number_of_tiles = self.tiles()[color as usize];
        self.tiles_mut()[color as usize] = 0;
        *self.size_mut() -= number_of_tiles as usize;

        number_of_tiles
    }
}
