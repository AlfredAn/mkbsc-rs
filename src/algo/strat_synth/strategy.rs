use crate::*;

#[derive(Debug, Clone)]
pub struct AllStrategies<const N: usize> {
    parts: [AllStrategies1; N]
}

#[derive(Debug, Clone)]
pub struct MlessStrat<S: MemorylessStrategy1, const N: usize>([S; N]);

impl<'a, S: MemorylessStrategy1, const N: usize> Strategy<N> for MlessStrat<S, N> {
    type M = ();

    fn call(&self, obs: Obs, _: &(), agt: Agt) -> Option<(Act, ())> {
        self.0[agt as usize].call1(obs, &())
    }
    fn init(&self) -> [Self::M; N] { [(); N] }
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
    
    pub fn get(&self) -> MlessStrat<MlessStrat1<Vec<Option<Act>>>, N> {
        MlessStrat(array_init(|i| self.parts[i].get()))
    }

    pub fn get_ref<'a>(&'a self) -> MlessStrat<MlessStrat1<&'a Vec<Option<Act>>>, N> {
        MlessStrat(array_init(|i| self.parts[i].get_ref()))
    }

    pub fn get_raw(&self) -> [&Vec<Option<Act>>; N] {
        array_init(|i| self.parts[i].get_raw())
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

pub fn all_strategies<T, const N: usize>(mkbsc: &MKBSC<T, N>) -> AllStrategies<N> {
    AllStrategies::new(
        array_init(|i|
            all_strategies1(&mkbsc.gki[i])
        )
    )
}

pub trait Strategy<const N: usize> {
    type M: Clone + Eq + Hash;
    fn call(&self, obs: Obs, mem: &Self::M, agt: Agt) -> Option<(Act, Self::M)>;
    fn init(&self) -> [Self::M; N];
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
