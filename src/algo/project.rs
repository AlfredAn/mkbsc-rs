use std::iter::Map;

use petgraph::visit::{GraphBase, Data, IntoEdgeReferences, IntoNeighbors, IntoEdges};

use crate::game::{MAGame, Game, MAGIIAN, IIGame};

#[derive(Clone, Copy)]
pub struct Project<'a, G: MAGame>(pub &'a G, pub G::AgentId);

impl<'a, G> GraphBase for Project<'a, G>
where
    G: MAGame
{
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

impl<'a, G> Game for Project<'a, G>
where
    G: MAGame + 'a
{
    type ActionId = G::AgentActId;
    type Actions<'b> where Self: 'b = impl Iterator<Item=Self::ActionId>;

    fn l0(&self) -> Self::NodeId {
        self.0.l0()
    }

    fn action(&self, e: Self::EdgeId) -> Self::Actions<'_> {
        self.0.action(e).map(|a| self.0.act_i(a, self.1))
    }

    fn is_winning(&self, n: Self::NodeId) -> bool {
        self.0.is_winning(n)
    }

    type Successors<'b> where Self: 'b = impl Iterator<Item=Self::EdgeId>;

    fn successors(&self, n: Self::NodeId) -> Self::Successors<'_> {
        self.0.successors(n)
    }

    fn source(&self, e: Self::EdgeId) -> Self::NodeId {
        self.0.source(e)
    }

    fn target(&self, e: Self::EdgeId) -> Self::NodeId {
        self.0.target(e)
    }
}

impl<'a, G> IIGame for Project<'a, G>
where
    G: MAGIIAN + 'a
{
    type ObsId = G::AgentObsId;

    fn observe(&self, l: Self::NodeId) -> Self::ObsId {
        self.0.obs_i(self.0.observe(l), self.1)
    }
}

derive_ma!(Project<'a, G>, 'a, G: MAGIIAN + 'a);
derive_magiian!(Project<'a, G>, 'a, G: MAGIIAN + 'a);
