use petgraph::graph::IndexType;

use super::index::ObsIndex;

#[derive(Debug, Clone, Copy)]
pub struct DNode<Ix: IndexType, const N_AGT: usize> {
    pub is_winning: bool,
    pub obs: [ObsIndex<Ix>; N_AGT]
}

impl<Ix: IndexType, const N_AGT: usize> DNode<Ix, N_AGT> {
    pub fn new(is_winning: bool, obs: [ObsIndex<Ix>; N_AGT]) -> Self {
        Self { is_winning: is_winning, obs: obs }
    }
}
