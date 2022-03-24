use petgraph::graph::IndexType;

use super::index::ObsIndex;

#[derive(Debug, Clone)]
pub struct DNode<Ix: IndexType, const N: usize> {
    pub is_winning: bool,
    pub obs: [ObsIndex<Ix>; N],
    pub debug: Option<String>
}

impl<Ix: IndexType, const N: usize> DNode<Ix, N> {
    pub fn new(is_winning: bool, obs: [ObsIndex<Ix>; N], debug: Option<String>) -> Self {
        Self { is_winning: is_winning, obs: obs, debug: debug }
    }
}
