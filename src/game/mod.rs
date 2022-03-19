use std::hash::Hash;

use itertools::Itertools;
use petgraph::{visit::{GraphBase}, graph::IndexType};

pub mod dgame;
pub mod strategy;
pub mod history;

#[macro_use]
pub mod macros;

pub trait Game {
    type Loc: Copy + Eq + Hash;
    type Act: Copy + Eq + Hash;
    type Obs: Copy + Eq + Hash;

    type Agent: IndexType;
    type AgentAct: Copy + Eq + Hash;
    type ActionsI<'a>: Iterator<Item=Self::AgentAct> where Self: 'a;

    type Actions<'a>: Iterator<Item=Self::Act> where Self: 'a;
    type Post<'a>: Iterator<Item=Self::Loc> where Self: 'a;

    fn l0(&self) -> Self::Loc;
    fn is_winning(&self, n: Self::Loc) -> bool;
    fn post(&self, n: Self::Loc, a: Self::Act) -> Self::Post<'_>;
    fn actions(&self) -> Self::Actions<'_>;

    fn observe(&self, l: Self::Loc) -> Self::Obs;

    fn n_agents(&self) -> usize;
    fn act_i(&self, act: Self::Act, agt: Self::Agent) -> Self::AgentAct;

    fn actions_i(&self, agt: Self::Agent) -> Self::ActionsI<'_>;

    type AgentObs: Copy + Eq + Hash;
    fn obs_i(&self, obs: Self::Obs, agt: Self::Agent) -> Self::AgentObs;
}

pub trait IIGame: Game {}
pub trait MAGame: Game {}

pub trait MAGIIAN: IIGame + MAGame {}
