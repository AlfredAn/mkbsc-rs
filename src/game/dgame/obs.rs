use std::fmt;

use array_init::array_init;

use itertools::Itertools;
use petgraph::graph::{IndexType, NodeIndex};

#[derive(Clone)]
pub struct DObs {
    pub set: Vec<NodeIndex>
}

impl DObs {
    pub fn new(set: Vec<NodeIndex>) -> Self {
        Self { set: set }
    }

    pub fn default_array<const N_AGT: usize>() -> [Vec<DObs>; N_AGT] {
        array_init(|_| Vec::new())
    }
}

impl Default for DObs {
    fn default() -> Self { Self::new(Vec::new()) }
}

impl fmt::Debug for DObs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.set.iter().map(|x| x.index()).format("|"))
    }
}
