use std::{collections::HashMap, hash::Hash, fmt::Debug, error::Error, iter};

use array_init::from_iter;
use petgraph::{graph::IndexType, visit::{Visitable, IntoEdges, VisitMap, EdgeRef, GraphBase}};

use crate::game::Game;

use super::{DGame, builder::Builder, generic_builder::GenericBuilder, index::agent_index, DGameType};

pub trait FromGame<'a, G, const N: usize>
where
    G: Game<'a, N>
{
    type Output: Game<'a, N>;
    type Err;

    fn from_game(g: &'a G, stop_on_win: bool) -> Result<Self::Output, Self::Err>;
}

impl<'a, G, DG, const N: usize> FromGame<'a, G, N> for DG
where
    G: Game<'a, N>,
    DG: DGameType<'a, N>
{
    type Output = DG;
    type Err = anyhow::Error;

    fn from_game(g: &'a G, stop_on_win: bool) -> Result<Self::Output, Self::Err> {
        let mut b = GenericBuilder::<_, _, (), N>::default();

        let mut stack = vec![g.l0()];
        while let Some(l) = stack.pop() {
            if b.has_node(l) {
                continue;
            }

            let is_winning = g.is_winning(l);
            b.add_node(l, is_winning)?;

            for a in g.actions() {
                for n in g.post(l, a) {
                    stack.push(n);
                    if !(stop_on_win && is_winning) {
                        b.add_edge(l, n, iter::once((0..N).map(|i| a[i])))?;
                    }
                }
            }
        }

        b.build()
    }
}
