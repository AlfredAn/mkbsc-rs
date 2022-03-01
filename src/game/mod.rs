use petgraph::{visit::{GraphBase}};

pub mod dgame;

pub trait Game: GraphBase {
    //type AgtCount: AgentCount;
    type ActionId: Copy + PartialEq;
    fn l0(&self) -> Self::NodeId;
}

pub trait IIGame: Game {
    type ObsId: Copy + PartialEq;
    fn observe(&self, l: Self::NodeId) -> Self::ObsId;
}

pub trait MAGame: Game {
    type AgentId: Copy + PartialEq;
    type AgentActId: Copy + PartialEq;
    fn n_agents(&self) -> usize;
    fn act_i(&self, act: Self::ActionId, agt: Self::AgentId) -> Self::AgentActId;
}

pub trait MAGIIAN: IIGame + MAGame {
    type AgentObsId: Copy + PartialEq;
    fn obs_i(&self, obs: Self::ObsId, agt: Self::AgentId) -> Self::AgentObsId;
}
/*
pub trait AgentCount {}
pub enum SingleAgent {}
pub enum MultiAgent {}
impl AgentCount for SingleAgent {}
impl AgentCount for MultiAgent {}

pub trait InformationType {}
pub enum PerfectInformation {}
pub enum ImperfectInformation {}
impl InformationType for PerfectInformation {}
impl InformationType for ImperfectInformation {}*/

impl<G: Game> Game for &G {
    //type AgtCount = G::AgtCount;
    type ActionId = G::ActionId;
    fn l0(&self) -> Self::NodeId { (*self).l0() }
}

impl<G: IIGame> IIGame for &G {
    type ObsId = G::ObsId;
    fn observe(&self, l: Self::NodeId) -> Self::ObsId { (*self).observe(l) }
}
