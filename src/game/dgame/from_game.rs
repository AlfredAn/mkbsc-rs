use std::{collections::HashMap, hash::Hash, fmt::Debug};

use array_init::from_iter;
use petgraph::{graph::IndexType, visit::{Visitable, IntoEdges, VisitMap, EdgeRef, GraphBase}};

use crate::game::{Game, MAGame};

use super::{DMAGIIAN, builder::Builder, generic_builder::GenericBuilder, index::agent_index};

impl<Ix: IndexType, const N_AGT: usize> DMAGIIAN<Ix, N_AGT> {
    pub fn from_game<'a, G>(g: &'a G, stop_on_win: bool) -> anyhow::Result<DMAGIIAN<Ix, N_AGT>>
    where
        G: MAGame + Visitable,
        &'a G: GraphBase<NodeId=G::NodeId, EdgeId=G::EdgeId> + IntoEdges,
        G::NodeId: Eq + Hash,
        G::ActionId: IntoIterator<Item=G::AgentActId>,
        G::AgentActId: Eq + Hash + Debug,
        G::AgentId: IndexType
    {
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

            for e in g.edges(l) {
                stack.push(e.target());
                if !(stop_on_win && is_winning) {
                    b.add_edge(
                        l, e.target(),
                        g.act(e.id())
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
