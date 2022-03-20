use std::{collections::HashMap, hash::Hash, fmt::Debug, error::Error, iter};

use array_init::from_iter;
use petgraph::{graph::IndexType, visit::{Visitable, IntoEdges, VisitMap, EdgeRef, GraphBase}};

use crate::game::Game;

use super::{DGame, builder::Builder, generic_builder::GenericBuilder, index::agent_index};

pub trait FromGame<'a, G, const N: usize>
where
    G: Game<'a, N>
{
    type Output: Game<'a, N>;
    type Err;

    fn from_game(g: &'a G, stop_on_win: bool) -> Result<Self::Output, Self::Err>;
}

impl<'a, Ix, G, const N: usize> FromGame<'a, G, N> for DGame<Ix, N>
where
    Ix: IndexType,
    G: Game<'a, N>
{
    type Output = DGame<Ix, N>;
    type Err = anyhow::Error;

    fn from_game(g: &'a G, stop_on_win: bool) -> Result<Self::Output, Self::Err> {
        let mut b = GenericBuilder::<_, _, (), N>::default();

        let mut stack = vec![g.l0().clone()];
        while let Some(l) = stack.pop() {
            if b.has_node(&l) {
                continue;
            }

            let is_winning = g.is_winning(&l);
            b.add_node(l.clone(), is_winning)?;

            for a in g.actions() {
                for n in g.post(&l, a) {
                    if !(stop_on_win && is_winning) {
                        b.add_edge(l.clone(), n.clone(), iter::once((0..N).map(|i| a[i])))?;
                    }
                    stack.push(n);
                }
            }
        }

        b.build()
    }
}
