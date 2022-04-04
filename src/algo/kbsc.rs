use std::collections::BTreeSet;
use std::borrow::Borrow;
use itertools::Itertools;

use crate::{game::*, util::{Itr, into_cloneable}};

#[derive(Clone, Debug)]
pub struct KBSC<'a, G: Game<'a, 1>, R: Borrow<G>> {
    pub g: R,
    l0: BTreeSet<G::Loc>
}

impl<'a, G, R> KBSC<'a, G, R>
where
    G: Game<'a, 1>,
    G::Loc: Ord,
    R: Borrow<G>,
{
    pub fn new(g: R) -> Self {
        Self {
            l0: BTreeSet::from([g.borrow().l0().clone()]),
            g: g
        }
    }
}

impl<'a, G, R> Game<'a, 1> for KBSC<'a, G, R>
where
    G: Game<'a, 1>,
    G::Loc: Ord,
    R: Borrow<G>
{
    type Loc = BTreeSet<G::Loc>;
    type Act = G::Act;

    fn l0<'b>(&'b self) -> &'b Self::Loc where 'a: 'b {
        &self.l0
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        n.iter().all(|l| self.g.borrow().is_winning(l))
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; 1]) -> Itr<'b, Self::Loc> where 'a: 'b {
        //this is inefficient, needs to be optimized
        let p = into_cloneable(self.g.borrow().post_set(n.iter(), a));
        let all_obs = p.clone().map(|l| self.g.borrow().observe(&l)).unique();
        let result = all_obs
            .map(move |o| p.clone()
                .filter(|l| self.g.borrow().observe(&l) == o)
                .collect()
            );
        Box::new(result)
    }

    fn actions<'b>(&'b self) -> Itr<'b, [Self::Act; 1]> where 'a: 'b {
        self.g.borrow().actions()
    }

    fn debug_string(&self, s: &Self::Loc) -> Option<String> {
        Some(format!("{{{}}}",
            s.iter()
                .map(|l| {
                    if let Some(d) = self.g.borrow().debug_string(l) {
                        d
                    } else {
                        "?".into()
                    }
                })
                .format(" | ")
        ))
    }

    derive_ii!(1);
    derive_ma!('a);
    derive_magiian!();
}

impl<'a, G, R> Game1<'a> for KBSC<'a, G, R>
where
    G: Game<'a, 1>,
    G::Loc: Ord,
    R: Borrow<G>
{}

//impl_ref!(KBSC<'a, G, R>, ('a, G: Game<'a, 1> + 'a), (where G::Loc: Ord), 1, {});
