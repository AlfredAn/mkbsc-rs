use std::fmt::Debug;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::collections::HashSet;
use itertools::__std_iter::once;
use crate::algo::*;
use crate::game::dgame::DGame;
use std::borrow::*;
use std::hash::Hash;

use petgraph::graph::IndexType;

use crate::util::{Itr};

pub mod dgame;

#[macro_use]
pub mod macros;

pub use dgame::*;

pub trait Game<const N: usize>: Debug {
    type Loc: Clone + Eq + Hash + Debug;
    type Act: Copy + Eq + Hash + Debug;
    type Obs: Clone + Eq + Hash + Debug;
    type Agt: IndexType;

    fn agent(i: usize) -> Self::Agt {
        Self::Agt::new(i)
    }

    fn l0(&self) -> &Self::Loc;
    fn is_winning(&self, n: &Self::Loc) -> bool;

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; N]) -> Itr<'b, Self::Loc>;

    fn actions(&self) -> Itr<[Self::Act; N]>;

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; N];
    fn observe_i(&self, l: &Self::Loc, agt: Self::Agt) -> Self::Obs {
        self.observe(l)[agt.index()].clone()
    }

    fn obs_eq(&self, l1: &Self::Loc, l2: &Self::Loc, agt: Self::Agt) -> bool {
        self.observe_i(l1, agt) == self.observe_i(l2, agt)
    }

    fn actions_i(&self, agt: Self::Agt) -> Itr<Self::Act> {
        Box::new(self.actions()
            .map(move |a| a[agt.index()])
            .unique()
        )
    }
    
    fn post_set<'b, I>(&'b self, ns: I, a: [Self::Act; N]) -> Itr<'b, Self::Loc>
    where
        I: IntoIterator<Item=&'b Self::Loc>,
        I::IntoIter: 'b
    {
        Box::new(ns.into_iter()
            .map(move |n| self.post(n, a))
            .flatten()
        )
    }
    
    fn debug_string(&self, _: &Self::Loc) -> Option<String> {
        None
    }

    fn dgame<'b>(&'b self) -> Cow<'b, DGame<N>> {
        Cow::Owned(DGame::from_game(self).dg)
    }

    fn into_dgame(self) -> DGame<N>
    where
        Self: Sized
    {
        self.dgame().into_owned()
    }
}

pub trait HasVisitSet<const N: usize>: Game<N> {
    type VisitSet: VisitSet<Self::Loc>;
    fn visit_set(&self) -> Self::VisitSet;
    fn visit_set_from_iterator<I: IntoIterator<Item=Self::Loc>>(&self, itr: I) -> Self::VisitSet {
        let mut s = self.visit_set();
        for l in itr.into_iter() {
            s.insert(l);
        }
        s
    }
    fn visit_set_singleton(&self, l: Self::Loc) -> Self::VisitSet {
        self.visit_set_from_iterator(once(l))
    }
}

pub trait VisitSet<Loc> {
    fn insert(&mut self, l: Loc) -> bool;
    fn clear(&mut self);
    fn contains(&self, l: &Loc) -> bool;
    //fn iter(&self) -> Itr<&Loc>;
}

impl<T: Eq + Hash> VisitSet<T> for HashSet<T> {
    fn insert(&mut self, l: T) -> bool {
        HashSet::insert(self, l)
    }
    fn clear(&mut self) { HashSet::clear(self) }
    fn contains(&self, l: &T) -> bool { HashSet::contains(self, l) }

    /*fn iter(&self) -> Itr<&T> {
        Box::new(HashSet::iter(self))
    }*/
}

impl<T: Ord> VisitSet<T> for BTreeSet<T> {
    fn insert(&mut self, l: T) -> bool {
        BTreeSet::insert(self, l)
    }
    fn clear(&mut self) { BTreeSet::clear(self) }
    fn contains(&self, l: &T) -> bool { BTreeSet::contains(self, l) }

    /*fn iter(&self) -> Itr<&T> {
        Box::new(BTreeSet::iter(self))
    }*/
}

pub trait Pre<'a, const N: usize>: Game<N> {
    fn pre<'b, I>(&'b self, ns: I, a: [Self::Act; N]) -> Itr<'b, Self::Loc>
    where I: IntoIterator<Item=&'b Self::Loc>, I::IntoIter: 'b;
}

pub trait Game1: Game<1> {
    fn agent1() -> Self::Agt {
        Self::agent(0)
    }

    fn actions1<'b>(&'b self) -> Itr<'b, Self::Act> {
        self.actions_i(Self::agent1())
    }

    fn observe1(&self, l: &Self::Loc) -> Self::Obs {
        self.observe_i(l, Self::agent1())
    }

    fn post1<'b>(&'b self, n: &'b Self::Loc, a: Self::Act) -> Itr<'b, Self::Loc> {
        self.post(n, [a])
    }

    fn post_set1<'b, I>(&'b self, ns: I, a: Self::Act) -> Itr<'b, Self::Loc>
    where
        I: IntoIterator<Item=&'b Self::Loc>,
        I::IntoIter: 'b
    {
        self.post_set(ns, [a])
    }

    fn all_strategies1(&self) -> AllStrategies1 {
        self.dgame().all_strategies1()
    }
}

pub trait GameRef<G: Game<N>, const N: usize>: Borrow<G> + Sized {
    fn project(self, agt: impl Into<G::Agt>) -> Project<G, Self, N> {
        Project(self, agt.into())
    }

    fn kbsc(self) -> KBSC<G, Self>
    where
        G: Game1,
        <G as Game<1>>::Loc: Ord
    {
        KBSC::new(self)
    }

    fn mkbsc(self) -> MKBSC<G, N>
    where
        Self: ToOwned<Owned=G>,
        G::Loc: Ord
    {
        MKBSC::new(self.to_owned())
    }
}

impl<G, R, const N: usize> GameRef<G, N> for R
where
    G: Game<N>,
    R: Borrow<G>
{}
