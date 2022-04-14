use crate::*;
use super::*;

#[derive(Debug, Clone)]
pub struct AllStrategies<const N: usize> {
    parts: [AllStrategies1; N]
}

impl<const N: usize> AllStrategies<N> {
    pub fn advance(&mut self) -> bool {
        for p in &mut self.parts {
            if p.advance() {
                return true;
            } else {
                p.reset();
            }
        }

        false
    }

    pub fn get(&self) -> [&Vec<Option<Act>>; N] {
        array_init(|i| self.parts[i].get())
    }

    pub fn reset(&mut self) {
        for p in &mut self.parts {
            p.reset();
        }
    }

    fn new(parts: [AllStrategies1; N]) -> Self {
        Self { parts }
    }
}

pub trait Strategy<const N: usize> {
    type M;
    fn call(&self, obs: Obs, mem: &Self::M, agt: Agt) -> Option<(Act, Self::M)>;
}

pub trait MemorylessStrategy<const N: usize>: Strategy<N, M=()> {
    fn call_ml(&self, obs: Obs, agt: Agt) -> Option<Act> {
        self.call(obs, &(), agt).map(|(a, _)| a)
    }
}

pub trait Strategy1: Strategy<1> {
    fn call1(&self, obs: Obs, mem: &Self::M) -> Option<(Act, Self::M)> {
        self.call(obs, mem, 0)
    }
}

pub trait MemorylessStrategy1:
    MemorylessStrategy<1>
     + Strategy1<M=()>
{
    fn call_ml1(&self, obs: Obs) -> Option<Act> {
        self.call_ml(obs, 0)
    }
}

impl<T, const N: usize> MemorylessStrategy<N> for T
where T: Strategy<N, M=()> {}

impl<T> Strategy1 for T
where T: Strategy<1> {}

impl<T> MemorylessStrategy1 for T
where T: MemorylessStrategy<1> + Strategy1<M=()> {}
