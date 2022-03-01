// mostly copy pasta from petgraph source code
#![allow(dead_code)]

use std::fmt;

use petgraph::graph::{DefaultIx, IndexType};

pub type NodeIndex = petgraph::graph::NodeIndex;
pub type EdgeIndex = petgraph::graph::EdgeIndex;

pub use petgraph::graph::{node_index, edge_index};

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AgentIndex<Ix = DefaultIx>(Ix);

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ActionIndex<Ix = DefaultIx>(Ix);

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ObsIndex<Ix = DefaultIx>(Ix);

impl<Ix: IndexType> AgentIndex<Ix> {
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

unsafe impl<Ix: IndexType> IndexType for AgentIndex<Ix> {
    fn index(&self) -> usize {
        self.0.index()
    }
    fn new(x: usize) -> Self {
        AgentIndex::new(x)
    }
    fn max() -> Self {
        AgentIndex(<Ix as IndexType>::max())
    }
}

impl<Ix: IndexType> From<Ix> for AgentIndex<Ix> {
    fn from(ix: Ix) -> Self {
        AgentIndex(ix)
    }
}

impl<Ix: fmt::Debug> fmt::Debug for AgentIndex<Ix> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AgentIndex({:?})", self.0)
    }
}

impl<Ix: IndexType> ActionIndex<Ix> {
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

unsafe impl<Ix: IndexType> IndexType for ActionIndex<Ix> {
    fn index(&self) -> usize {
        self.0.index()
    }
    fn new(x: usize) -> Self {
        ActionIndex::new(x)
    }
    fn max() -> Self {
        ActionIndex(<Ix as IndexType>::max())
    }
}

impl<Ix: IndexType> From<Ix> for ActionIndex<Ix> {
    fn from(ix: Ix) -> Self {
        ActionIndex(ix)
    }
}

impl<Ix: fmt::Debug> fmt::Debug for ActionIndex<Ix> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActionIndex({:?})", self.0)
    }
}

impl<Ix: IndexType> ObsIndex<Ix> {
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

unsafe impl<Ix: IndexType> IndexType for ObsIndex<Ix> {
    fn index(&self) -> usize {
        self.0.index()
    }
    fn new(x: usize) -> Self {
        ObsIndex::new(x)
    }
    fn max() -> Self {
        ObsIndex(<Ix as IndexType>::max())
    }
}

impl<Ix: IndexType> From<Ix> for ObsIndex<Ix> {
    fn from(ix: Ix) -> Self {
        ObsIndex(ix)
    }
}

impl<Ix: fmt::Debug> fmt::Debug for ObsIndex<Ix> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ObsIndex({:?})", self.0)
    }
}

/// Short version of `ActionIndex::new`
pub fn action_index<Ix: IndexType>(index: usize) -> ActionIndex<Ix> {
    ActionIndex::new(index)
}

/// Short version of `ObsIndex::new`
pub fn obs_index<Ix: IndexType>(index: usize) -> ObsIndex<Ix> {
    ObsIndex::new(index)
}
