use std::hash::Hash;

use itertools::Itertools;
use petgraph::{visit::{GraphBase}, graph::IndexType};

pub mod dgame;
pub mod strategy;
pub mod history;

#[macro_use]
pub mod macros;

pub trait Game<'a, const N: usize> {
    type Loc: Copy + Eq + Hash;
    type Act: Copy + Eq + Hash;
    type Obs: Copy + Eq + Hash;
    type Agent: IndexType;

    type Actions: Iterator<Item=[Self::Act; N]>;
    type ActionsI: Iterator<Item=Self::Act>;
    type Post: Iterator<Item=Self::Loc>;

    fn l0(&self) -> Self::Loc;
    fn is_winning(&self, n: Self::Loc) -> bool;
    fn post(&'a self, n: Self::Loc, a: [Self::Act; N]) -> Self::Post;
    fn actions(&'a self) -> Self::Actions;

    fn observe(&self, l: Self::Loc) -> [Self::Obs; N];

    fn actions_i(&'a self, agt: Self::Agent) -> Self::ActionsI;
}
