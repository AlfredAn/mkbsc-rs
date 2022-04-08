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
pub struct AgentIndex(u8);

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ActionIndex(u16);

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ObsIndex(u32);

impl AgentIndex {
    #[inline]
    pub fn new(x: usize) -> Self {
        AgentIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self {
        AgentIndex(IndexType::max())
    }
}

unsafe impl IndexType for AgentIndex {
    fn index(&self) -> usize {
        self.0.index()
    }
    fn new(x: usize) -> Self {
        AgentIndex::new(x)
    }
    fn max() -> Self {
        AgentIndex(u8::MAX)
    }
}

impl From<usize> for AgentIndex {
    fn from(t: usize) -> Self {
        Self::new(t)
    }
}

impl fmt::Debug for AgentIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AgentIndex({:?})", self.0)
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
pub fn agent_index(index: usize) -> AgentIndex {
    AgentIndex::new(index)
}

/// Short version of `ActionIndex::new`
pub fn action_index(index: usize) -> ActionIndex {
    ActionIndex::new(index)
}

/// Short version of `ObsIndex::new`
pub fn obs_index(index: usize) -> ObsIndex {
    ObsIndex::new(index)
}
