use std::{hash::Hash, ops::Deref};

use itertools::Itertools;
use petgraph::{visit::{GraphBase}, graph::IndexType};
use std::fmt::Debug;

use crate::util::{Itr};

pub mod dgame;
pub mod strategy;
pub mod history;

#[macro_use]
pub mod macros;

pub trait Game<'a, const N: usize>: Debug {
    type Loc: Clone + Eq + Hash + Debug;
    type Act: Copy + Eq + Hash + Debug + 'a;
    type Agent: IndexType;

    fn agent(i: usize) -> Self::Agent {
        Self::Agent::new(i)
    }

    fn l0<'b>(&'b self) -> &'b Self::Loc where 'a: 'b;
    fn is_winning(&self, n: &Self::Loc) -> bool;
    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; N]) -> Itr<'b, Self::Loc> where 'a: 'b;

    fn actions<'b>(&'b self) -> Itr<'b, [Self::Act; N]> where 'a: 'b;

    fn obs_eq(&self, l1: &Self::Loc, l2: &Self::Loc, agt: Self::Agent) -> bool;

    fn actions_i<'b>(&'b self, agt: Self::Agent) -> Itr<'b, Self::Act> where 'a: 'b {
        Box::new(self.actions()
            .map(move |a| a[agt.index()])
            .unique())
    }
    
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

pub trait ObsSet<'a, const N: usize>: Game<'a, N> {
    fn obs_set(&self, l: &Self::Loc, agt: Self::Agent) -> Itr<Self::Loc>;
}

pub trait Game1<'a>: Game<'a, 1> {
    const AGENT: Self::Agent;
}

impl<'a, G: Game<'a, 1>> Game1<'a> for G {
    const AGENT: Self::Agent = Self::agent(0);
}

impl<'a, R, G, const N: usize> Game<'a, N> for R
where
    G: Game<'a, N> + 'a,
    R: Deref<Target=G> + Debug
{
    type Loc = G::Loc;
    type Act = G::Act;
    type Agent = G::Agent;

    fn l0<'b>(&'b self) -> &'b Self::Loc where 'a: 'b {
        self.deref().l0()
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        self.deref().is_winning(n)
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; N]) -> Itr<'b, Self::Loc> where 'a: 'b {
        self.deref().post(n, a)
    }

    fn actions<'b>(&'b self) -> Itr<'b, [Self::Act; N]> where 'a: 'b {
        self.deref().actions()
    }

    fn actions_i<'b>(&'b self, agt: Self::Agent) -> Itr<'b, Self::Act> where 'a: 'b {
        self.deref().actions_i(agt)
    }
    
    fn post_set<'b, I>(&'b self, ns: I, a: [Self::Act; N]) -> Itr<'b, Self::Loc>
    where
        I: IntoIterator<Item=&'b Self::Loc>,
        I::IntoIter: 'b,
        'a: 'b
    {
        self.deref().post_set(ns, a)
    }

    fn debug_string(&self, l: &Self::Loc) -> Option<String> {
        self.deref().debug_string(l)
    }
}
