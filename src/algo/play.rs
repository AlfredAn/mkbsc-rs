#![allow(dead_code)]

use std::{marker::PhantomData};

use petgraph::visit::{Walker, IntoEdges, EdgeRef};
use itertools::Itertools;

use crate::game::{strategy::MemorylessStrategy, Game};

pub struct Play<'a, N, A, S>
where
    N: Copy + PartialEq,
    A: Copy + PartialEq,
    S: MemorylessStrategy<N, A>
{
    loc: N,
    strat: &'a S,
    first: bool,

    _a: PhantomData<A>
}

#[derive(Debug, Clone, Copy)]
pub struct PlayStep<N, A, E>
where
    N: Copy + PartialEq,
    A: Copy + PartialEq,
    E: Copy + PartialEq
{
    to: N,
    step: Option<PlayStepEdge<N, A, E>>
}

impl<N, A, E> From<(N, Option<(N, A, E)>)> for PlayStep<N, A, E>
where
    N: Copy + PartialEq,
    A: Copy + PartialEq,
    E: Copy + PartialEq
{
    fn from(x: (N, Option<(N, A, E)>)) -> Self {
        Self { to: x.0, step: x.1.map_or(None, |y| Some(y.into())) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayStepEdge<N, A, E>
where
    N: Copy + PartialEq,
    A: Copy + PartialEq,
    E: Copy + PartialEq
{
    from: N,
    act: A,
    edge: E
}

impl<N, A, E> From<(N, A, E)> for PlayStepEdge<N, A, E>
where
    N: Copy + PartialEq,
    A: Copy + PartialEq,
    E: Copy + PartialEq
{
    fn from(x: (N, A, E)) -> Self {
        Self { from: x.0, act: x.1, edge: x.2 }
    }
}

impl<'a, N, A, S> Play<'a, N, A, S>
where
    N: Copy + PartialEq,
    A: Copy + PartialEq,
    S: MemorylessStrategy<N, A>
{
    pub fn new(l: N, s: &'a S) -> Self {
        Self { loc: l, strat: s, first: true, _a: PhantomData }
    }

    pub fn from_game<G: Game<NodeId=N, ActionId=A>>(g: &G, s: &'a S) -> Self {
        Self::new(g.l0(), s)
    }
}

impl<'a, N, A, E, G, S> Walker<&'a G> for Play<'a, N, A, S>
where
    N: Copy + PartialEq,  
    A: Copy + PartialEq,
    E: Copy + PartialEq,
    &'a G: Game<NodeId=N, ActionId=A, EdgeId=E> + IntoEdges,
    S: MemorylessStrategy<N, A>
{
    type Item = PlayStep<N, A, E>;

    fn walk_next(&mut self, g: &'a G) -> Option<PlayStep<N, A, E>> {
        let l = self.loc;

        if self.first {
            self.first = false;
            return Some((l, None).into());
        }
        
        let act = self.strat[l];
        let edge = g.edges(l).find(|x| g.act(x.id()).contains(&act));

        if let Some(e) = edge {
            self.loc = e.target();
            Some((e.target(), Some((l, act, e.id()))).into())
        } else {
            None
        }
    }
}

pub fn until_win<'a, I, G>(i: I, g: &'a G) -> impl Iterator<Item=PlayStep<G::NodeId, G::ActionId, G::EdgeId>> + 'a
where
    I: Iterator<Item=PlayStep<G::NodeId, G::ActionId, G::EdgeId>> + 'a,
    G: Game
{
    i.take_while(|&x| if let Some(y) = x.step {
        !g.is_winning(y.from)
    } else {
        true
    })
}
