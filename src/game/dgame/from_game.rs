use std::{collections::HashMap, hash::Hash, fmt::Debug, error::Error, iter};

use array_init::from_iter;
use petgraph::{graph::IndexType, visit::{Visitable, IntoEdges, VisitMap, EdgeRef, GraphBase}};

use crate::game::{Game, MAGame, MAGIIAN};

use super::{DGame, builder::Builder, generic_builder::GenericBuilder, index::agent_index, DGameType};

pub trait FromGame<'a, G, const N_AGT: usize>
where
    G: Game
{
    type Output: Game;
    type Err;

    fn from_game(g: &G, stop_on_win: bool) -> Result<Self::Output, Self::Err>;
}

impl<'a, G, DG, const N_AGT: usize> FromGame<'a, G, N_AGT> for DG
where
    G: MAGIIAN,
    G::Loc: Eq + Hash,
    G::AgentAct: Eq + Hash,
    G::Agent: IndexType,
    DG: DGameType<N_AGT>
{
    type Output = DG;
    type Err = anyhow::Error;

    fn from_game(g: &G, stop_on_win: bool) -> Result<Self::Output, Self::Err> {
        if g.n_agents() != N_AGT {
            anyhow::bail!("Wrong number of agents");
        }

        let mut b = GenericBuilder::<_, _, (), N_AGT>::default();

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
                        b.add_edge(l, n, iter::once((0..g.n_agents()).map(|i| g.act_i(a, G::Agent::new(i)))))?;
                    }
                }
            }
        }

        b.build()
    }
}
