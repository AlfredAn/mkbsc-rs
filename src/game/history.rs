use std::ops::Index;

use super::*;

pub trait History<G: Game>: Index<usize, Output=G::NodeId> {
    fn len(&self) -> usize;
    fn loc(&self, i: usize) -> G::NodeId {
        self[i]
    }
}

pub trait ObsHistory<G: IIGame>: History<G> {
    fn obs(&self, i: usize, g: &G) -> G::ObsId;
}

impl<G: Game> History<G> for Vec<G::NodeId> {
    fn len(&self) -> usize { self.len() }
}

impl<G: IIGame, H: History<G>> ObsHistory<G> for H {
    fn obs(&self, i: usize, g: &G) -> G::ObsId {
        g.observe(self[i])
    }
}
