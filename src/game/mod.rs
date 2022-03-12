use petgraph::{visit::{GraphBase}};

pub mod dgame;
pub mod strategy;
pub mod history;

#[macro_use]
pub mod macros;

pub trait Game: GraphBase {
    type AgtCount: AgentCount;
    type InfoType: InformationType;

    type ActionId: Copy + PartialEq;

    type Actions<'a>: Iterator<Item=Self::ActionId> where Self: 'a;

    fn l0(&self) -> Self::NodeId;
    fn act(&self, e: Self::EdgeId) -> Self::Actions<'_>;
    fn is_winning(&self, n: Self::NodeId) -> bool;
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

pub trait AgentCount {}
pub enum SingleAgent {}
pub enum MultiAgent {}
impl AgentCount for SingleAgent {}
impl AgentCount for MultiAgent {}

pub trait InformationType {}
pub enum PerfectInformation {}
pub enum ImperfectInformation {}
impl InformationType for PerfectInformation {}
impl InformationType for ImperfectInformation {}

impl<G: Game<InfoType=PerfectInformation>> IIGame for G {
    type ObsId = Self::NodeId;
    fn observe(&self, l: Self::NodeId) -> Self::ObsId { l }
}

impl<G: Game<AgtCount=SingleAgent>> MAGame for G {
    type AgentId = ();
    type AgentActId = Self::ActionId;
    fn n_agents(&self) -> usize { 1 }
    fn act_i(&self, act: Self::ActionId, _: Self::AgentId) -> Self::AgentActId { act }
}

impl<G: Game<InfoType=PerfectInformation, AgtCount=SingleAgent>> MAGIIAN for G {
    type AgentObsId = Self::ObsId;
    fn obs_i(&self, obs: Self::ObsId, _: Self::AgentId) -> Self::AgentObsId { obs }
}

impl<G: Game> Game for &G {
    type AgtCount = G::AgtCount;
    type InfoType = G::InfoType;
    type ActionId = G::ActionId;
    type Actions<'a> where G: 'a, Self: 'a = G::Actions<'a>;
    fn l0(&self) -> Self::NodeId { (*self).l0() }
    fn act(&self, e: Self::EdgeId) -> Self::Actions<'_> { (*self).act(e) }
    fn is_winning(&self, n: Self::NodeId) -> bool { (*self).is_winning(n) }
}
