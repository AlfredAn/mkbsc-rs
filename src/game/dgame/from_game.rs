use std::{collections::HashMap, hash::Hash, fmt::Debug, error::Error};

use array_init::from_iter;
use petgraph::{graph::IndexType, visit::{Visitable, IntoEdges, VisitMap, EdgeRef, GraphBase}};

use crate::game::{Game, MAGame, MAGIIAN};

use super::{DMAGIIAN, builder::Builder, generic_builder::GenericBuilder, index::agent_index, DGameType};

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
    G::NodeId: Eq + Hash,
    G::AgentActId: Eq + Hash,
    G::AgentId: IndexType,
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

            for e in g.successors(l) {
                stack.push(g.target(e));
                if !(stop_on_win && is_winning) {
                    b.add_edge(
                        l, g.target(e),
                        g.action(e)
                            .map(|act| (0..N_AGT).map(move |i|
                                g.act_i(act, G::AgentId::new(i))
                            )
                        )
                    )?;
                }
            }
        }

        b.build()
    }
}
