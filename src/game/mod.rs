use std::hash::Hash;

use itertools::Itertools;
use petgraph::{visit::{GraphBase}, graph::IndexType};

pub mod dgame;
pub mod strategy;
pub mod history;

#[macro_use]
pub mod macros;

pub trait Game<'a, const N: usize> {
    type Loc: Clone + Eq + Hash;
    type Act: Copy + Eq + Hash;
    type Obs: Clone + Eq + Hash;
    type Agent: IndexType;

    type Actions: Iterator<Item=[Self::Act; N]>;
    type ActionsI: Iterator<Item=Self::Act>;
    type Post<'b>: Iterator<Item=Self::Loc> where Self: 'b;

    fn l0(&self) -> &Self::Loc;
    fn is_winning(&self, n: &Self::Loc) -> bool;
    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; N]) -> Self::Post<'b>;
    fn actions(&self) -> Self::Actions;

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; N];

    fn actions_i(&self, agt: Self::Agent) -> Self::ActionsI;
}
