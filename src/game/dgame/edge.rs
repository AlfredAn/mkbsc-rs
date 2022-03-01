use petgraph::graph::IndexType;

use super::index::ActionIndex;

#[derive(Debug)]
pub struct DEdge<Ix: IndexType, const N_AGT: usize> {
    pub act: Vec<[ActionIndex<Ix>; N_AGT]>
}

impl<Ix: IndexType, const N_AGT: usize> DEdge<Ix, N_AGT> {
    pub fn new(act: Vec<[ActionIndex<Ix>; N_AGT]>) -> Self {
        Self { act: act }
    }
}

impl<Ix: IndexType, const N_AGT: usize> Default for DEdge<Ix, N_AGT> {
    fn default() -> Self { Self::new(Vec::new()) }
}
