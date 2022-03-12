#![allow(dead_code)]

use std::ops::{Index, Deref, DerefMut, IndexMut};

use petgraph::graph::IndexType;

pub trait MemorylessStrategy<N, A>: Index<N, Output=A> {}

impl<N, A, C> MemorylessStrategy<N, A> for C
where
    N: Copy + PartialEq,
    A: Copy + PartialEq,
    C: Index<N, Output=A>
{}

pub struct VecStrat<A: Copy + PartialEq>(Vec<A>);

impl<A: IndexType> VecStrat<A> {
    pub fn new<F>(f: F) -> Self
    where
        F: IntoIterator<Item=usize>
    {
        Self(f.into_iter().map(|x| A::new(x)).collect())
    }
}

impl<I: IndexType> VecStrat<[I; 1]> {
    pub fn new1<F>(f: F) -> Self
    where
        F: IntoIterator<Item=usize>
    {
        Self(f.into_iter().map(|x| [I::new(x)]).collect())
    }
}

impl<A, F> From<F> for VecStrat<A>
where
    A: Copy + PartialEq,
    F: Into<Vec<A>>
{
    fn from(v: F) -> Self {
        Self(v.into())
    }
}

impl<N, A> Index<N> for VecStrat<A>
where
    N: IndexType,
    A: Copy + PartialEq
{
    type Output = A;

    fn index(&self, index: N) -> &A {
        &self.0[index.index()]
    }
}

impl<N, A> IndexMut<N> for VecStrat<A>
where
    N: IndexType,
    A: Copy + PartialEq
{
    fn index_mut(&mut self, index: N) -> &mut A {
        &mut self.0[index.index()]
    }
}

impl<A> Deref for VecStrat<A>
where
    A: Copy + PartialEq
{
    type Target = Vec<A>;

    fn deref(&self) -> &Vec<A> {
        &self.0
    }
}

impl<A> DerefMut for VecStrat<A>
where
    A: Copy + PartialEq
{
    fn deref_mut(&mut self) -> &mut Vec<A> {
        &mut self.0
    }
}
