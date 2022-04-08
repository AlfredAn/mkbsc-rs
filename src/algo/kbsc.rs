use lazycell::LazyCell;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::borrow::Borrow;
use itertools::Itertools;

use crate::{game::*, dgame::*, util::{Itr, into_cloneable}};

#[derive(Clone, Debug)]
pub struct KBSC<G: Game<1>, R: Borrow<G>> {
    pub g: R,
    dg: LazyCell<DGame<1>>,
    l0: BTreeSet<G::Loc>
}

impl<G, R> KBSC<G, R>
where
    G: Game<1>,
    G::Loc: Ord,
    R: Borrow<G>,
{
    pub fn new(g: R) -> Self {
        Self {
            l0: BTreeSet::from([g.borrow().l0().clone()]),
            dg: LazyCell::new(),
            g: g
        }
    }
}

impl<G, R> Game<1> for KBSC<G, R>
where
    G: Game<1>,
    G::Loc: Ord,
    R: Borrow<G>
{
    type Loc = BTreeSet<G::Loc>;
    type Act = G::Act;

    fn l0(&self) -> &Self::Loc {
        &self.l0
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        n.iter().all(|l| self.g.borrow().is_winning(l))
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; 1]) -> Itr<'b, Self::Loc> {
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

    fn actions(&self) -> Itr<[Self::Act; 1]> {
        self.g.borrow().actions()
    }

    fn dgame(&self) -> Cow<DGame<1>> {
        Cow::Borrowed(
            self.dg.borrow_with(|| DGame::from_game(self, false).unwrap())
        )
    }

    fn into_dgame(self) -> DGame<1> {
        self.dgame();
        self.dg.into_inner().unwrap()
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
    derive_ma!();
    derive_magiian!();
}

impl<G, R> Game1 for KBSC<G, R>
where
    G: Game<1>,
    G::Loc: Ord,
    R: Borrow<G>
{}

//impl_ref!(KBSC<G, R>, ('a, G: Game<1> + 'a), (where G::Loc: Ord), 1, {});
