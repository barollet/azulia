pub use crate::rules::*;

pub struct RoundManager {
    draw_bag: DrawBag,
    discard_pile: DiscardPile,

    n_fabrics: usize,
    fabrics: Vec<Fabric>,

    ground_pile: Fabric, // the discarded tiles of this round
}

impl RoundManager {
    /// Creates a new round manager provided a number of fabrics
    pub fn new(fabrics: usize) -> Self {
        let mut rm = Self {
            draw_bag: DrawBag::init_draw_bag(),
            discard_pile: DiscardPile::init_discard_pile(),

            n_fabrics: fabrics,
            fabrics: vec![Default::default(); fabrics],

            ground_pile: Default::default(),
        };

        rm.initialize_new_round();

        rm
    }

    pub fn initialize_new_round(&mut self) {
        // picks tiles from the draw bag, if the draw bag does not contains enough tiles, reuse the
        // discard pile
        for _ in 0..self.n_fabrics {
            let mut fabric = Fabric::default();

            while fabric.size() < 4 {
                let tile = self.draw_bag.next().unwrap_or_else(|| {
                    // replace the draw bag with the discard pile
                    self.draw_bag =
                        std::mem::replace(&mut self.discard_pile, DiscardPile::init_discard_pile());

                    self.draw_bag.next().unwrap()
                });
                fabric.add_single_tile(tile);
            }

            self.fabrics.push(fabric);
        }
    }
}
