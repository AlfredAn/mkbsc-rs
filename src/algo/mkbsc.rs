use crate::Game1;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::{rc::Rc, iter::{once, self}};

use crate::{game::{Game, dgame::index::agent_index}, util::{Itr, iterator_product}};
use array_init::*;
use itertools::{izip, Itertools};
use super::{kbsc::KBSC, project::Project};
use petgraph::adj::IndexType;

type K<'a, G, const N: usize> = KBSC<'a, Project<'a, G, Rc<G>, N>, Project<'a, G, Rc<G>, N>>;
type KLoc<'a, G, const N: usize> = <K<'a, G, N> as Game<'a, 1>>::Loc;

#[derive(Clone)]
pub struct MKBSC<'a, G, const N: usize>
where
    G: Game<'a, N> + 'a,
    G::Loc: Ord
{
    pub g: Rc<G>,
    pub kbsc: [K<'a, G, N>; N],
    l0: [KLoc<'a, G, N>; N],
}

impl<'a, G, const N: usize> MKBSC<'a, G, N>
where
    G: Game<'a, N>,
    G::Loc: Ord
{
    pub fn new(g: G) -> Self {
        let g = Rc::new(g);
        let kbsc = array_init(|i|
            K::new(
                Project(g.clone(), G::Agent::new(i))
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

impl<'a, G, const N: usize> Game<'a, N> for MKBSC<'a, G, N>
where
    G: Game<'a, N> + 'a,
    G::Loc: Ord
{
    type Loc = [KLoc<'a, G, N>; N];
    type Act = G::Act;
    type Obs = KLoc<'a, G, N>;
    type Agent = G::Agent;

    fn l0<'b>(&'b self) -> &'b Self::Loc where 'a: 'b {
        &self.l0
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        izip!(n, &self.kbsc).any(|(s, g)|
            g.is_winning(s)
        )
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; N]) -> Itr<'b, Self::Loc> where 'a: 'b {
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

    fn actions<'b>(&'b self) -> Itr<'b, [Self::Act; N]> where 'a: 'b {
        self.g.actions()
    }

    fn actions_i<'b>(&'b self, agt: Self::Agent) -> Itr<'b, Self::Act> where 'a: 'b {
        self.g.actions_i(agt)
    }
    
    fn observe_i(&self, l: &Self::Loc, agt: Self::Agent) -> Self::Obs {
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
}

impl<'a, G> Game1<'a> for MKBSC<'a, G, 1>
where
    G: Game<'a, 1>,
    G::Loc: Ord
{}
