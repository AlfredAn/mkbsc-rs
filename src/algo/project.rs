use std::borrow::Borrow;
use crate::game::Game1;

use petgraph::graph::IndexType;

use crate::{game::Game, util::Itr};

#[derive(Clone, Copy, Debug)]
pub struct Project<G: Game<N>, R: Borrow<G>, const N: usize>(pub R, pub G::Agt);

impl<G, R, const N: usize> Game<1> for Project<G, R, N>
where
    G: Game<N>,
    R: Borrow<G>
{
    type Loc = G::Loc;
    type Act = G::Act;
    type Obs = G::Obs;

    fn l0(&self) -> &Self::Loc {
        self.0.borrow().l0()
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        self.0.borrow().is_winning(n)
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; 1]) -> Itr<'b, Self::Loc> {
        Box::new(self.0.borrow().actions()
            .filter(move |&aa| aa[self.1.index()] == a[0])
            .map(move |aa| self.0.borrow().post(n, aa))
            .flatten()
        )
    }

    fn actions(&self) -> Itr<[Self::Act; 1]> {
        Box::new(self.0.borrow().actions_i(self.1).map(|a| [a]))
    }

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; 1] {
        [self.0.borrow().observe(l)[self.1.index()].clone()]
    }

    derive_ma!();
    derive_magiian!();

    fn debug_string(&self, l: &Self::Loc) -> Option<String> {
        self.0.borrow().debug_string(l)
    }
}

impl<G, R, const N: usize> Project<G, R, N>
where
    G: Game<N>,
    R: Borrow<G>
{
    pub fn sub_actions(&self, a: G::Act) -> Itr<[G::Act; N]> {
        Box::new(self.0.borrow().actions()
            .filter(move |aa| aa[self.1.index()] == a)
        )
    }
}

impl<G, R, const N: usize> Game1 for Project<G, R, N>
where
    G: Game<N>,
    R: Borrow<G>
{}
