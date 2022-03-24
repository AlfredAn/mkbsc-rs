use std::iter::{Map, self};

use itertools::Itertools;
use petgraph::{visit::{GraphBase, Data, IntoEdgeReferences, IntoNeighbors, IntoEdges}, graph::IndexType};
use array_init::array_init;

use crate::{game::Game, util::Itr};

#[derive(Clone, Copy, Debug)]
pub struct Project<'a, G: Game<'a, N>, const N: usize>(pub G, pub G::Agent);

impl<'a, G, const N: usize> Game<'a, 1> for Project<'a, G, N>
where
    G: Game<'a, N>
{
    type Loc = G::Loc;
    type Act = G::Act;
    type Obs = G::Obs;

    fn l0<'b>(&'b self) -> &'b Self::Loc where 'a: 'b {
        self.0.l0()
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        self.0.is_winning(n)
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; 1]) -> Itr<Self::Loc> where 'a: 'b {
        Box::new(self.0.actions()
            .filter(move |&aa| aa[self.1.index()] == a[0])
            .map(move |aa| self.0.post(n, aa))
            .flatten()
            .unique())
    }

    fn actions<'b>(&'b self) -> Itr<[Self::Act; 1]> where 'a: 'b {
        Box::new(self.0.actions_i(self.1).map(|a| [a]))
    }

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; 1] {
        [self.0.observe(l)[self.1.index()].clone()]
    }

    derive_ma!('a);
    derive_magiian!();

    fn debug_string(&self, l: &Self::Loc) -> Option<String> {
        self.0.debug_string(l)
    }
}
