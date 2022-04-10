use std::fmt::Debug;
use std::fmt::Display;

use super::index::ObsIndex;

#[derive(Debug, Clone)]
pub struct DNode<T: Clone, const N: usize> {
    pub is_winning: bool,
    pub obs: [ObsIndex; N],
    pub data: T
}

impl<T: Clone, const N: usize> DNode<T, N> {
    pub fn new(is_winning: bool, obs: [ObsIndex; N], data: T) -> Self {
        Self { is_winning, obs, data }
    }
}

impl<T: Clone, const N: usize> Display for DNode<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        if false {//let Some(s) = &self.debug {
            //write!(f, "{}", s)?;
        } else {
            write!(f, "?")?;
        }

        Ok(())
    }
}
