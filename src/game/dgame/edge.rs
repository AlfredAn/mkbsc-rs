use petgraph::graph::IndexType;

use super::index::ActionIndex;

#[derive(Debug)]
pub struct DEdge<Ix: IndexType> {
    pub act: Vec<ActionIndex<Ix>>
}

impl<Ix: IndexType> DEdge<Ix> {
    pub fn new(act: Vec<ActionIndex<Ix>>) -> Self {
        Self { act: act }
    }
}

impl<Ix: IndexType> Default for DEdge<Ix> {
    fn default() -> Self { Self::new(Vec::new()) }
}
