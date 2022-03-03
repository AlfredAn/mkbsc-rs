use std::ops::Index;

pub trait MemorylessStrategy<N, A>: Index<N, Output=A> {}

impl<N, A, C> MemorylessStrategy<N, A> for C
where
    N: Copy + PartialEq,
    A: Copy + PartialEq,
    C: Index<N, Output=A>
{}
