use std::fmt::Display;
use std::fmt;

use itertools::Itertools;
use petgraph::graph::IndexType;

use super::index::ActionIndex;

#[derive(Clone)]
pub struct DEdge<Ix: IndexType, const N: usize> {
    pub act: Vec<[ActionIndex<Ix>; N]>
}

impl<Ix: IndexType, const N: usize> DEdge<Ix, N> {
    pub fn new(act: Vec<[ActionIndex<Ix>; N]>) -> Self {
        Self { act: act }
    }
}

impl<Ix: IndexType, const N: usize> Default for DEdge<Ix, N> {
    fn default() -> Self { Self::new(Vec::new()) }
}

impl<Ix: IndexType, const N: usize> fmt::Debug for DEdge<Ix, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.act.iter()
            .format_with("|", |agt, f| f(&format_args!("{}", agt.iter()
                .map(|x| x.index()).format(".")
            )))
        )
    }
}

impl<Ix: IndexType, const N: usize> Display for DEdge<Ix, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
