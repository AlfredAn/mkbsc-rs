use std::ops::Index;

use super::*;

pub trait History<N: Copy + PartialEq>: Index<usize, Output=N> {
    fn len(&self) -> usize;
    fn loc(&self, i: usize) -> N {
        self[i]
    }
}

pub trait ObsHistory<G: IIGame>: History<G::Loc> {
    fn obs(&self, i: usize, g: &G) -> G::Obs;
}

impl<N: Copy + PartialEq> History<N> for Vec<N> {
    fn len(&self) -> usize { self.len() }
}

impl<G: IIGame, H: History<G::Loc>> ObsHistory<G> for H {
    fn obs(&self, i: usize, g: &G) -> G::Obs {
        g.observe(self[i])
    }
}
