use petgraph::{visit::{GraphBase}, graph::IndexType};

pub mod dgame;
pub mod strategy;
pub mod history;

#[macro_use]
pub mod macros;

pub trait Game: GraphBase {
    type ActionId: Copy + PartialEq;

    type Actions<'a>: Iterator<Item=Self::ActionId> where Self: 'a;
    type Successors<'a>: Iterator<Item=Self::EdgeId> where Self: 'a;

    fn l0(&self) -> Self::NodeId;
    fn action(&self, e: Self::EdgeId) -> Self::Actions<'_>;
    fn source(&self, e: Self::EdgeId) -> Self::NodeId;
    fn target(&self, e: Self::EdgeId) -> Self::NodeId;
    fn is_winning(&self, n: Self::NodeId) -> bool;
    fn successors(&self, n: Self::NodeId) -> Self::Successors<'_>;
}

pub trait IIGame: Game {
    type ObsId: Copy + PartialEq;
    fn observe(&self, l: Self::NodeId) -> Self::ObsId;
}

pub trait MAGame: Game {
    type AgentId: IndexType;
    type AgentActId: Copy + PartialEq;
    fn n_agents(&self) -> usize;
    fn act_i(&self, act: Self::ActionId, agt: Self::AgentId) -> Self::AgentActId;
}

pub trait MAGIIAN: IIGame + MAGame {
    type AgentObsId: Copy + PartialEq;
    fn obs_i(&self, obs: Self::ObsId, agt: Self::AgentId) -> Self::AgentObsId;
}

impl<G: Game> Game for &G {
    type ActionId = G::ActionId;
    type Actions<'a> where G: 'a, Self: 'a = G::Actions<'a>;
    fn l0(&self) -> Self::NodeId { (*self).l0() }
    fn action(&self, e: Self::EdgeId) -> Self::Actions<'_> { (*self).action(e) }
    fn is_winning(&self, n: Self::NodeId) -> bool { (*self).is_winning(n) }

    type Successors<'a> where Self: 'a = G::Successors<'a>;

    fn successors(&self, n: Self::NodeId) -> Self::Successors<'_> {
        (*self).successors(n)
    }

    fn source(&self, e: Self::EdgeId) -> Self::NodeId {
        (*self).source(e)
    }

    fn target(&self, e: Self::EdgeId) -> Self::NodeId {
        (*self).target(e)
    }
}
