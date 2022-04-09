use std::fmt::*;
use itertools::Itertools;
use super::index::ActionIndex;

#[derive(Clone)]
pub struct DEdge<const N: usize> {
    pub act: [ActionIndex; N]
}

impl<const N: usize> DEdge<N> {
    pub fn new(act: [ActionIndex; N]) -> Self {
        Self { act: act }
    }
}

impl<const N: usize> Debug for DEdge<N> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.act.iter()
            .map(|x| x.index())
            .format(".")
        )
    }
}

/*impl<const N: usize> Display for DEdge<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}*/
