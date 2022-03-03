use std::marker::PhantomData;

use petgraph::visit::{Walker, IntoNeighbors, GraphBase, IntoEdges, EdgeRef};
use itertools::Itertools;

use crate::game::{strategy::MemorylessStrategy, Game};

pub struct Play<N, A, S>
where
    N: Copy + PartialEq,
    A: Copy + PartialEq,
    S: MemorylessStrategy<N, A>
{
    loc: N,
    strat: S,

    _a: PhantomData<A>
}

impl<'a, N, A, G, S> Walker<&'a G> for Play<N, A, S>
where
    N: Copy + PartialEq,  
    A: Copy + PartialEq,
    &'a G: Game<NodeId=N, ActionId=A> + IntoEdges,
    S: MemorylessStrategy<N, A>
{
    type Item = N;

    fn walk_next(&mut self, g: &'a G) -> Option<N> {
        let l = self.loc;
        let act = self.strat[l];
        let edge = g.edges(l).find(|x| g.act(x.id()).contains(&act));

        if let Some(e) = edge {
            self.loc = e.target();
            Some(l)
        } else {
            None
        }
    }
}
