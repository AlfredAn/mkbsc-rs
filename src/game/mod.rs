use std::borrow::Borrow;
use std::borrow::Cow;
use crate::util::MaybeRef;
use crate::algo::strat_synth::strategy::AllStrategies;
use crate::find_memoryless_strategies;
use crate::AllStrategies1;
use crate::KBSC;
use crate::Project;
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

pub trait Game<'a, const N: usize> {
    type Loc: Clone + Eq + Hash;
    type Act: Copy + Eq + Hash + 'a;
    type Obs: Clone + Eq + Hash;
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

    fn dgame<'b>(&'b self) -> Cow<'b, DGame<N>> where 'a: 'b {
        Cow::Owned(DGame::from_game(self, false).unwrap())
    }

    fn into_dgame(self) -> DGame<N>
    where
        Self: Sized
    {
        self.dgame().into_owned()
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

    fn all_strategies(&self) -> AllStrategies1 {
        self.dgame().all_strategies()
    }
}

pub trait GameRef<'a, G: Game<'a, N>, const N: usize>: Borrow<G> + Sized {
    fn project(self, agt: impl Into<G::Agent>) -> Project<'a, G, Self, N> {
        Project(self, agt.into())
    }

    fn kbsc(self) -> KBSC<'a, G, Self>
    where
        G: Game1<'a>,
        <G as Game<'a, 1>>::Loc: Ord
    {
        KBSC::new(self)
    }

    fn mkbsc(self) -> MKBSC<'a, G, N>
    where
        Self: ToOwned<Owned=G>,
        G::Loc: Ord
    {
        MKBSC::new(self.to_owned())
    }
}

impl<'a, G, R, const N: usize> GameRef<'a, G, N> for R
where
    G: Game<'a, N>,
    R: Borrow<G>
{}
