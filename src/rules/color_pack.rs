//! A collection for holding groups of color. This is a basic common struct for the draw bag,
//! discard pile and fabrics

use crate::rules::NB_COLORS;

// Number of remaining tiles for each color in the bag
// Wrapper type around an array
#[derive(Clone, Debug)]
pub struct ColorPack {
    tiles: [u8; NB_COLORS],
    size: usize,
}

impl ColorPack {
    pub fn init_sized_bag(color_size: u8) -> Self {
        Self {
            tiles: [color_size; NB_COLORS],
            size: color_size as usize * NB_COLORS,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn size_mut(&mut self) -> &mut usize {
        &mut self.size
    }

    pub fn tiles(&self) -> &[u8; NB_COLORS] {
        &self.tiles
    }

    pub fn tiles_mut(&mut self) -> &mut [u8; NB_COLORS] {
        &mut self.tiles
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn add_single_tile(&mut self, color: u8) {
        self.tiles[color as usize] += 1;
        self.size += 1;
    }

    pub fn remove_single_tile(&mut self, color: u8) {
        self.tiles[color as usize] -= 1;
        self.size -= 1;
    }
}

impl Default for ColorPack {
    fn default() -> Self {
        Self::init_sized_bag(0)
    }
}
