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

pub trait Game<const N: usize> {
    type Loc: Clone + Eq + Hash;
    type Act: Copy + Eq + Hash;
    type Obs: Clone + Eq + Hash;
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
        Cow::Owned(DGame::from_game(self, false).unwrap())
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
}

pub trait VisitSet<Loc> {
    fn insert(&mut self, l: impl Borrow<Loc> + ToOwned<Owned=Loc>) -> bool;
    fn clear(&mut self);
    fn contains(&self, l: impl Borrow<Loc>) -> bool;

    fn insert_clone(&mut self, l: impl Borrow<Loc>) -> bool
    where
        Loc: Clone
    {
        let l = l.borrow();
        if self.contains(l) {
            false
        } else {
            self.insert(l.clone());
            true
        }
    }

    fn try_insert<Q>(&mut self, l: Q) -> Option<Q>
    where
        Q: Borrow<Loc> + ToOwned<Owned=Loc>
    {
        if self.contains(l.borrow()) {
            Some(l)
        } else {
            self.insert(l);
            None
        }
    }
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
