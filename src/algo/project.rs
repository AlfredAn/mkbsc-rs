use std::iter::{Map, self};

use itertools::Itertools;
use petgraph::{visit::{GraphBase, Data, IntoEdgeReferences, IntoNeighbors, IntoEdges}, graph::IndexType};

use crate::game::Game;

#[derive(Clone, Copy)]
pub struct Project<'a, G: Game<'a, N>, const N: usize>(pub &'a G, pub G::Agent);

impl<'a, G, const N: usize> Game<'a, 1> for Project<'a, G, N>
where
    G: Game<'a, N>
{
    type Loc = G::Loc;
    type Act = G::Act;

    fn l0(&self) -> Self::Loc {
        self.0.l0()
    }

    fn is_winning(&self, n: Self::Loc) -> bool {
        self.0.is_winning(n)
    }

    type Post = impl Iterator<Item=Self::Loc>;

    fn post(&'a self, n: Self::Loc, a: [Self::Act; 1]) -> Self::Post {
        self.0.actions()
            .filter(move |&aa| aa[self.1.index()] == a[0])
            .map(move |aa| self.0.post(n, aa))
            .flatten()
            .unique()
    }

    type Actions = impl Iterator<Item=[Self::Act; 1]>;

    fn actions(&self) -> Self::Actions {
        self.0.actions_i(self.1).map(|a| [a])
    }

    type Obs = G::AgentObs;

    fn observe(&self, l: Self::Loc) -> Self::Obs {
        self.0.obs_i(self.0.observe(l), self.1)
    }

    derive_ma!('a);
    derive_magiian!();
}
