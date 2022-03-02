use std::fmt;


use itertools::Itertools;
use petgraph::graph::{IndexType, NodeIndex};

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

impl<Ix: IndexType> fmt::Debug for DObs<Ix> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.set.iter().map(|x| x.index()).format("|"))
    }
}
