use std::fmt;

use itertools::Itertools;
use petgraph::graph::IndexType;

use super::index::ActionIndex;

#[derive(Clone)]
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

impl<Ix: IndexType, const N_AGT: usize> fmt::Debug for DEdge<Ix, N_AGT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.act.iter()
            .format_with("|", |agt, f| f(&format_args!("{}", agt.iter()
                .map(|x| x.index()).format(".")
            )))
        )
    }
}
