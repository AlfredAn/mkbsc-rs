use std::collections::BTreeSet;
use std::{rc::Rc};

use array_init::*;
use itertools::{izip, Itertools};
use super::*;
use crate::{game::*, util::*};
use petgraph::adj::IndexType;

type K<G, const N: usize> = KBSC<Project<G, Rc<G>, N>, Project<G, Rc<G>, N>>;
type KLoc<G, const N: usize> = <K<G, N> as Game<1>>::Loc;

#[derive(Debug, Clone)]
pub struct MKBSC<G, const N: usize>
where
    G: Game<N>,
    G::Loc: Ord
{
    pub g: Rc<G>,
    pub kbsc: [K<G, N>; N],
    l0: [KLoc<G, N>; N],
}

impl<G, const N: usize> MKBSC<G, N>
where
    G: Game<N>,
    G::Loc: Ord
{
    pub fn new(g: G) -> Self {
        let g = Rc::new(g);
        let kbsc = array_init(|i|
            K::new(
                Project(g.clone(), G::Agt::new(i))
            )
        );
        let l0 = array_init(|i|
            kbsc[i].l0().clone()
        );

        Self {
            g: g,
            kbsc: kbsc,
            l0: l0
        }
    }
}

impl<G, const N: usize> Game<N> for MKBSC<G, N>
where
    G: Game<N>,
    G::Loc: Ord
{
    type Loc = [KLoc<G, N>; N];
    type Act = G::Act;
    type Obs = KLoc<G, N>;
    type Agt = G::Agt;

    fn l0(&self) -> &Self::Loc {
        &self.l0
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        izip!(n, &self.kbsc).any(|(s, g)|
            g.is_winning(s)
        )
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; N]) -> Itr<'b, Self::Loc> {
        let mut itr = n.iter();
        let intersection = itr.next()
            .map(|set| itr.fold(set.clone(), |s1, s2| &s1 & s2)).unwrap();
        let filter_set: BTreeSet<_> = self.g.post_set(intersection.iter(), a).collect();
        
        Box::new(iterator_product!(
            from_iter(
                izip!(n, a, &self.kbsc)
                    .map(|(n, a, k)| k.post(n, [a]))
            ).unwrap()
        ).filter(move |s| {
            let mut itr = s.iter();
            let intersection = itr.next()
                .map(|set| itr.fold(set.clone(), |s1, s2| &s1 & s2)).unwrap();
            !filter_set.is_disjoint(&intersection)
        }))
    }

    fn actions(&self) -> Itr<[Self::Act; N]> {
        self.g.actions()
    }

    fn actions_i(&self, agt: Self::Agt) -> Itr<Self::Act> {
        self.g.actions_i(agt)
    }
    
    fn observe_i(&self, l: &Self::Loc, agt: Self::Agt) -> Self::Obs {
        l[agt.index()].clone()
    }

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; N] {
        l.clone()
    }

    fn debug_string(&self, s: &Self::Loc) -> Option<String> {
        Some(format!("{}",
            (0..N).map(|i| {
                let (si, k) = (&s[i], &self.kbsc[i]);
                k.debug_string(si).unwrap()
            })
            .format(", ")
        ))
    }

    derive_dgame!(N);
}

impl<G> Game1 for MKBSC<G, 1>
where
    G: Game<1>,
    G::Loc: Ord
{}
