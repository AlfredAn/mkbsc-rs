use std::{rc::Rc, iter::{once, self}};

use crate::{game::{Game, dgame::index::agent_index}, util::{iterator_product, Itr}};
use array_init::*;
use itertools::{izip, Itertools};
use super::{kbsc::KBSC, project::Project};
use petgraph::adj::IndexType;

type K<'a, G, const N: usize> = KBSC<'a, Project<'a, Rc<G>, N>>;
type KLoc<'a, G, const N: usize> = <K<'a, G, N> as Game<'a, 1>>::Loc;

#[derive(Clone, Debug)]
pub struct MKBSC<'a, G, const N: usize>
where
    G: Game<'a, N> + 'a,
    G::Loc: Ord
{
    pub g: Rc<G>,
    kbsc: [K<'a, G, N>; N],
    l0: [KLoc<'a, G, N>; N]
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
    //type Obs = ();
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
        /*iterator_product(
            from_iter(
                izip!(n, a, &self.kbsc)
                    .map(|(n, a, k)| k.post(n, [a]))
            ).unwrap()
        )*/
        Box::new([n.clone()].into_iter())
    }

    fn actions<'b>(&'b self) -> Itr<'b, [Self::Act; N]> where 'a: 'b {
        self.g.actions()
    }

    fn actions_i<'b>(&'b self, agt: Self::Agent) -> Itr<'b, Self::Act> where 'a: 'b {
        self.g.actions_i(agt)
    }

    derive_ii!(N);

    fn post_set<'b, I>(&'b self, ns: I, a: [Self::Act; N]) -> Itr<'b, Self::Loc>
    where
        I: IntoIterator<Item=&'b Self::Loc>,
        I::IntoIter: 'b,
        'a: 'b
    {
        Box::new(ns.into_iter()
            .map(move |n| self.post(&n, a))
            .flatten()
            .unique())
    }

    fn debug_string(&self, _: &Self::Loc) -> Option<String> {
        None
    }
}
