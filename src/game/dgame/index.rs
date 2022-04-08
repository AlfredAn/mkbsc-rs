// mostly copy pasta from petgraph source code
#![allow(dead_code)]

use std::fmt;

pub use petgraph::graph::{DefaultIx, IndexType};

pub type NodeIndex = petgraph::graph::NodeIndex<u32>;
pub type EdgeIndex = petgraph::graph::EdgeIndex<u32>;

pub use petgraph::graph::{node_index, edge_index};

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct ZeroIndex();

unsafe impl IndexType for ZeroIndex {
    fn index(&self) -> usize {
        0
    }
    fn new(_: usize) -> Self {
        Self()
    }
    fn max() -> Self {
        Self()
    }
}

impl From<()> for ZeroIndex {
    fn from(_: ()) -> Self { Self() }
}

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AgtIndex(u8);

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ActionIndex(u16);

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ObsIndex(u32);

impl AgtIndex {
    #[inline]
    pub fn new(x: usize) -> Self {
        AgtIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self {
        AgtIndex(IndexType::max())
    }
}

unsafe impl IndexType for AgtIndex {
    fn index(&self) -> usize {
        self.0.index()
    }
    fn new(x: usize) -> Self {
        AgtIndex::new(x)
    }
    fn max() -> Self {
        AgtIndex(u8::MAX)
    }
}

impl From<usize> for AgtIndex {
    fn from(t: usize) -> Self {
        Self::new(t)
    }
}

impl fmt::Debug for AgtIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AgtIndex({:?})", self.0)
    }
}

impl ActionIndex {
    #[inline]
    pub fn new(x: usize) -> Self {
        ActionIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self {
        ActionIndex(IndexType::max())
    }
}

unsafe impl IndexType for ActionIndex {
    fn index(&self) -> usize {
        self.0.index()
    }
    fn new(x: usize) -> Self {
        ActionIndex::new(x)
    }
    fn max() -> Self {
        ActionIndex(u16::MAX)
    }
}

impl From<usize> for ActionIndex {
    fn from(t: usize) -> Self {
        Self::new(t)
    }
}

impl fmt::Debug for ActionIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActionIndex({:?})", self.0)
    }
}

impl ObsIndex {
    #[inline]
    pub fn new(x: usize) -> Self {
        ObsIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self {
        ObsIndex(IndexType::max())
    }
}

unsafe impl IndexType for ObsIndex {
    fn index(&self) -> usize {
        self.0.index()
    }
    fn new(x: usize) -> Self {
        ObsIndex::new(x)
    }
    fn max() -> Self {
        ObsIndex(u32::MAX)
    }
}

impl From<usize> for ObsIndex {
    fn from(t: usize) -> Self {
        Self::new(t)
    }
}

impl fmt::Debug for ObsIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ObsIndex({:?})", self.0)
    }
}

/// Short version of `ObsIndex::new`
pub fn agent_index(index: usize) -> AgtIndex {
    AgtIndex::new(index)
}

/// Short version of `ActionIndex::new`
pub fn action_index(index: usize) -> ActionIndex {
    ActionIndex::new(index)
}

/// Short version of `ObsIndex::new`
pub fn obs_index(index: usize) -> ObsIndex {
    ObsIndex::new(index)
}
