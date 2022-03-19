use std::iter::{Map, self};

use itertools::Itertools;
use petgraph::visit::{GraphBase, Data, IntoEdgeReferences, IntoNeighbors, IntoEdges};

use crate::game::{MAGame, Game, MAGIIAN, IIGame};

#[derive(Clone, Copy)]
pub struct Project<'a, G: MAGame>(pub &'a G, pub G::Agent);

impl<'a, G> Game for Project<'a, G>
where
    G: MAGIIAN + 'a
{
    type Loc = G::Loc;
    type Act = G::AgentAct;

    fn l0(&self) -> Self::Loc {
        self.0.l0()
    }

    fn is_winning(&self, n: Self::Loc) -> bool {
        self.0.is_winning(n)
    }

    type Post<'b> where Self: 'b = impl Iterator<Item=Self::Loc>;

    fn post(&self, n: Self::Loc, a: Self::Act) -> Self::Post<'_> {
        self.0.actions()
            .filter(move |&aa| self.0.act_i(aa, self.1) == a)
            .map(move |aa| self.0.post(n, aa))
            .flatten()
            .unique()
    }

    type Actions<'b> where Self: 'b = impl Iterator<Item=Self::Act>;

    fn actions(&self) -> Self::Actions<'_> {
        self.0.actions_i(self.1)
    }

    type Obs = G::AgentObs;

    fn observe(&self, l: Self::Loc) -> Self::Obs {
        self.0.obs_i(self.0.observe(l), self.1)
    }

    derive_ma!();
    derive_magiian!();
}

impl<'a, G> IIGame for Project<'a, G> where G: MAGIIAN + 'a {}
impl<'a, G> MAGame for Project<'a, G> where G: MAGIIAN + 'a {}
impl<'a, G> MAGIIAN for Project<'a, G> where G: MAGIIAN + 'a {}
