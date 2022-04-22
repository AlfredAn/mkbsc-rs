use std::iter;

use crate::*;

#[derive(Debug, Clone)]
pub struct AllStrategies<const N: usize> {
    parts: [AllStrategies1; N]
}

impl<const N: usize> AllStrategies<N> {
    pub fn new(g: [&Game<1>; N]) -> Self {
        let parts = g.map(|gi| all_strategies1(gi));
        Self {
            parts
        }
    }

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

    pub fn get(&self) -> [MlessStrat; N] {
        array_init(|i| self.parts[i].get())
    }

    pub fn get_raw(&self) -> [&Vec<Option<Act>>; N] {
        array_init(|i| self.parts[i].get_raw())
    }

    pub fn reset(&mut self) {
        for p in &mut self.parts {
            p.reset();
        }
    }

    pub fn into_iter(mut self) -> impl Iterator<Item=[MlessStrat; N]> {
        let mut first = true;
        let mut done = false;
        iter::from_fn(move || {
            if first {
                first = false;
                Some(self.get())
            } else {
                if !done && self.advance() {
                    Some(self.get())
                } else {
                    done = true;
                    None
                }
            }
        })
    }
}

pub fn all_strategies<const N: usize>(g: [&Game<1>; N]) -> AllStrategies<N> {
    AllStrategies::new(g)
}

pub trait Strategy {
    type M: Clone + Eq + Hash;
    fn call(&self, obs: Obs, mem: &Self::M) -> Option<(Act, Self::M)>;
    fn init(&self) -> Self::M;

    fn transducer(&self, g: &Game<1>) -> Transducer {
        Transducer::build(g, self)
    }

    fn transducer_ma<const N: usize>(&self, g: &Game<N>, agt: Agt) -> Transducer {
        Transducer::build_ma(g, agt, self)
    }
}

pub trait MemorylessStrategy: Strategy<M=()> {
    fn call_ml(&self, obs: Obs) -> Option<Act> {
        self.call(obs, &()).map(|(a, _)| a)
    }
}

impl<S> MemorylessStrategy for S
where S: Strategy<M=()> {}

impl<S: Strategy, R: Deref<Target=S>> Strategy for R {
    type M = S::M;
    fn call(&self, obs: Obs, mem: &S::M) -> Option<(Act, S::M)> {
        (**self).call(obs, mem)
    }
    fn init(&self) -> S::M {
        (**self).init()
    }
}

struct FnStrat<M, F: Fn(Obs, &M) -> Option<(Act, M)>>(F, M);

impl<M: Clone + Eq + Hash, F: Fn(Obs, &M) -> Option<(Act, M)>> Strategy for FnStrat<M, F> {
    type M = M;
    fn call(&self, obs: Obs, mem: &M) -> Option<(Act, M)> {
        (self.0)(obs, mem)
    }
    fn init(&self) -> M { self.1.clone() }
}

pub fn strategy<M: Clone + Eq + Hash>(
    init: M,
    f: impl Fn(Obs, &M) -> Option<(Act, M)>
) -> impl Strategy {
    FnStrat(f, init)
}
