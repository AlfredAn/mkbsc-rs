use std::collections::{HashSet, BTreeSet};

use crate::game::*;

#[derive(Clone)]
pub struct KBSC<'a, G: Game<1>> {
    pub g: &'a G,
    l0: BTreeSet<G::Loc>
}

impl<'a, G: Game<1>> KBSC<'a, G>
where
    G::Loc: Ord
{
    pub fn new(g: &'a G) -> Self {
        Self {
            g: g,
            l0: BTreeSet::from([g.l0().clone()])
        }
    }
}

impl<'a, G> Game<1> for KBSC<'a, G>
where
    G: Game<1>
{
    type Loc = BTreeSet<G::Loc>;
    type Act = G::Act;

    type Actions = impl Iterator<Item=[Self::Act; 1]>;
    type Post<'b> where Self: 'b = impl Iterator<Item=Self::Loc>;

    fn l0(&self) -> &Self::Loc {
        &self.l0
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        n.iter().all(|l| self.g.is_winning(l))
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; 1]) -> Self::Post<'b> {
        [].into_iter()
    }

    fn actions(&self) -> Self::Actions {
        self.g.actions()
    }

    post_set!(1, 'a);
    derive_ii!(1);
    derive_ma!('a);
    derive_magiian!();
}
