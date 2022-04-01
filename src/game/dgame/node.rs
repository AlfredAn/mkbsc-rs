use std::fmt::Display;
use petgraph::graph::IndexType;

use super::index::ObsIndex;

#[derive(Debug, Clone)]
pub struct DNode<const N: usize> {
    pub is_winning: bool,
    pub obs: [ObsIndex; N],
    pub debug: Option<String>
}

impl<const N: usize> DNode<N> {
    pub fn new(is_winning: bool, obs: [ObsIndex; N], debug: Option<String>) -> Self {
        Self { is_winning: is_winning, obs: obs, debug: debug }
    }
}

impl<const N: usize> Display for DNode<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if let Some(s) = &self.debug {
            write!(f, "{}", s)?;
        } else {
            write!(f, "?")?;
        }

        Ok(())
    }
}
