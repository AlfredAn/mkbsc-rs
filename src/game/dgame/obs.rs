use petgraph::graph::{IndexType, NodeIndex};

#[derive(Debug)]
pub struct DObs<Ix: IndexType> {
    pub set: Vec<NodeIndex<Ix>>
}

impl<Ix: IndexType> DObs<Ix> {
    pub fn new(set: Vec<NodeIndex<Ix>>) -> Self {
        Self { set: set }
    }
}

impl<Ix: IndexType> Default for DObs<Ix> {
    fn default() -> Self { Self::new(Vec::new()) }
}
