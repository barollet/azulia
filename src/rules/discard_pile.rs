use crate::rules::color_pack::*;
use crate::rules::fabric::*;

pub type DiscardPile = ColorPack;

impl ColorPack {
    /// Creates empty discard pile when a new game start
    pub fn init_discard_pile() -> Self {
        Default::default()
    }

    /// Adds the remaining of a fabric to the discard pile
    /// This consumes the fabric
    pub fn discard_fabric(&mut self, fabric: Fabric) {
        for (s, fs) in self.tiles_mut().iter_mut().zip(fabric.tiles()) {
            // adds the content of each color of the fabric to the discard pile
            *s += fs;
        }

        *self.size_mut() += fabric.size();
    }
}
