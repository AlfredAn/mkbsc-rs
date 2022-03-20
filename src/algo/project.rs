use std::iter::{Map, self};

use itertools::Itertools;
use petgraph::{visit::{GraphBase, Data, IntoEdgeReferences, IntoNeighbors, IntoEdges}, graph::IndexType};
use array_init::array_init;

use crate::game::Game;

#[derive(Clone, Copy)]
pub struct Project<'a, 'b, G: Game<'a, N>, const N: usize>(pub &'b G, pub G::Agent);

impl<'a, 'b, G: Game<'a, N>, const N: usize> Project<'a, 'b, G, N> {
    fn post_set<'c>(&'c self, n: &'c G::Loc, a: [G::Act; 1]) -> impl Iterator<Item=<G as Game<'a, N>>::Loc> + 'c
    
    {
        let t = self.0.actions()
            .filter(move |&aa| aa[self.1.index()] == a[0])
            .map(move |aa| self.0.post(n, aa))
            .flatten()
            .unique();
        t
        /*use itertools::Itertools;
        ns.into_iter()
            .map(move |n| self.post(n, a))
            .flatten()
            .unique()*/
    }
}

impl<'a, 'b, G, const N: usize> Game<'a, 1> for Project<'a, 'b, G, N>
where
    G: Game<'a, N>
{
    type Loc = G::Loc;
    type Act = G::Act;
    type Obs = G::Obs;

    fn l0(&self) -> &Self::Loc {
        self.0.l0()
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        self.0.is_winning(n)
    }

    type Post<'c> where Self: 'c = impl Iterator<Item=Self::Loc>;
    fn post<'c>(&'c self, n: &'c Self::Loc, a: [Self::Act; 1]) -> Self::Post<'c> {
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

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; 1] {
        [self.0.observe(l)[self.1.index()].clone()]
    }

    //post_set!(1);
    derive_ma!('a);
    derive_magiian!();
}


