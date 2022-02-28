use petgraph::graph::IndexType;

use super::index::ObsIndex;

#[derive(Debug)]
pub struct DNode<Ix: IndexType> {
    pub is_winning: bool,
    pub obs: ObsIndex<Ix>
}

impl<Ix: IndexType> DNode<Ix> {
    pub fn new(is_winning: bool, obs: ObsIndex<Ix>) -> Self {
        Self { is_winning: is_winning, obs: obs }
    }
}
