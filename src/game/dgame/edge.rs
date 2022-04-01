use std::fmt::Display;
use std::fmt;

use itertools::Itertools;
use petgraph::graph::IndexType;

use super::index::ActionIndex;

#[derive(Clone)]
pub struct DEdge<const N: usize> {
    pub act: Vec<[ActionIndex; N]>
}

impl<const N: usize> DEdge<N> {
    pub fn new(act: Vec<[ActionIndex; N]>) -> Self {
        Self { act: act }
    }
}

impl<const N: usize> Default for DEdge<N> {
    fn default() -> Self { Self::new(Vec::new()) }
}

impl<const N: usize> fmt::Debug for DEdge<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.act.iter()
            .format_with("|", |agt, f| f(&format_args!("{}", agt.iter()
                .map(|x| x.index()).format(".")
            )))
        )
    }
}

impl<const N: usize> Display for DEdge<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
