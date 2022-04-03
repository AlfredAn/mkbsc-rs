use crate::KBSC;
use crate::Project;
use crate::dgame;
use crate::DGame;
use crate::MKBSC;
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

pub trait Game<'a, const N: usize>: Debug where Self: 'a {
    type Loc: Clone + Eq + Hash + Debug;
    type Act: Copy + Eq + Hash + Debug + 'a;
    type Obs: Clone + Eq + Hash + Debug;
    type Agent: IndexType;

    fn agent(i: usize) -> Self::Agent {
        Self::Agent::new(i)
    }

    fn l0<'b>(&'b self) -> &'b Self::Loc where 'a: 'b;
    fn is_winning(&self, n: &Self::Loc) -> bool;

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; N]) -> Itr<'b, Self::Loc> where 'a: 'b;

    fn actions<'b>(&'b self) -> Itr<'b, [Self::Act; N]> where 'a: 'b;

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; N];
    fn observe_i(&self, l: &Self::Loc, agt: Self::Agent) -> Self::Obs {
        self.observe(l)[agt.index()].clone()
    }

    fn obs_eq(&self, l1: &Self::Loc, l2: &Self::Loc, agt: Self::Agent) -> bool {
        self.observe_i(l1, agt) == self.observe_i(l2, agt)
    }

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
            .map(move |n| self.post(n, a))
            .flatten()
            .unique())
    }
    
    fn debug_string(&self, _: &Self::Loc) -> Option<String> {
        None
    }

    fn project(self, agt: impl Into<Self::Agent>) -> Project<'a, Self, N>
    where
        Self: Sized
    {
        Project(self, agt.into())
    }

    fn kbsc(self) -> KBSC<'a, Self>
    where
        Self: Sized + Game1<'a>,
        <Self as Game<'a, 1>>::Loc: Ord
    {
        KBSC::new(self)
    }

    fn mkbsc(self) -> MKBSC<'a, Self, N>
    where
        Self: Sized,
        Self::Loc: Ord
    {
        MKBSC::new(self)
    }

    fn dgame(self) -> DGame<N>
    where
        Self: Sized
    {
        dgame(self)
    }
}

pub trait Pre<'a, const N: usize>: Game<'a, N> {
    fn pre<'b, I>(&'b self, ns: I, a: [Self::Act; N]) -> Itr<'b, Self::Loc>
    where 'a: 'b, I: IntoIterator<Item=&'b Self::Loc>, I::IntoIter: 'b;
}

pub trait Game1<'a>: Game<'a, 1> {
    fn agent1() -> Self::Agent {
        Self::agent(0)
    }

    fn actions1<'b>(&'b self) -> Itr<'b, Self::Act> where 'a: 'b {
        self.actions_i(Self::agent1())
    }

    fn post1<'b>(&'b self, n: &'b Self::Loc, a: Self::Act) -> Itr<'b, Self::Loc> where 'a: 'b {
        self.post(n, [a])
    }

    /*fn cpre<'b>(&'b self, n: &'b Self::Loc) -> Itr<'b, Self::Loc> where 'a: 'b;

    fn cpre_set<'b, I>(&'b self, ns: I) -> Itr<'b, Self::Loc>
    where 'a: 'b, I: IntoIterator<Item=&'b Self::Loc>, I::IntoIter: 'b {
        Box::new(ns.into_iter()
            .map(|n| self.cpre(&n))
            .flatten()
            .unique()
        )
    }*/
}

impl<'a, G: Game<'a, 1>> Game1<'a> for G {}

impl<'a, R, G, const N: usize> Game<'a, N> for R
where
    G: Game<'a, N>,
    R: Deref<Target=G> + Debug + 'a
{
    type Loc = G::Loc;
    type Act = G::Act;
    type Obs = G::Obs;
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

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; N] {
        self.deref().observe(l)
    }

    fn observe_i(&self, l: &Self::Loc, agt: Self::Agent) -> Self::Obs {
        self.deref().observe_i(l, agt)
    }

    fn obs_eq(&self, l1: &Self::Loc, l2: &Self::Loc, agt: Self::Agent) -> bool {
        self.deref().obs_eq(l1, l2, agt)
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

impl<'a, R, G, const N: usize> Pre<'a, N> for R
where
    G: Pre<'a, N>,
    R: Deref<Target=G> + Debug + 'a
{
    fn pre<'b, I>(&'b self, ns: I, a: [Self::Act; N]) -> Itr<'b, Self::Loc>
    where 'a: 'b, I: IntoIterator<Item=&'b Self::Loc>, I::IntoIter: 'b {
        self.deref().pre(ns, a)
    }
}
