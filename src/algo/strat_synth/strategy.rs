use std::borrow::Borrow;
use crate::algo::KBSC;
use crate::algo::MKBSC;
use crate::game::dgame::DGame;
use crate::game::*;
use array_init::array_init;
use std::iter;
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

    pub fn get(&self) -> [&Vec<Option<ActionIndex>>; N] {
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

    pub fn iter<'b>(&'b mut self) -> impl Iterator<Item=[Vec<Option<ActionIndex>>; N]> + 'b {
        let mut finished = false;
        let mut first = true;
        iter::from_fn(move || {
            if finished {
                return None;
            } else if !first {
                if !self.advance() {
                    finished = true;
                    return None;
                }
            } else {
                first = false;
            }
            Some(self.get().map(|x| x.clone()))
        })
    }
}

pub fn all_strategies<const N: usize>(g: [&DGame<1>; N]) -> AllStrategies<N> {
    AllStrategies::new(
        g.map(|g| g.all_strategies1())
    )
}

impl<G, R> KBSC<G, R>
where
    G: Game1 + HasVisitSet<1>,
    G::Loc: Ord,
    R: Borrow<G>
{
    /*pub fn translate_strategy(
        &self,
        strat: impl MemorylessStrategy1<<Self as Game<1>>::Obs, G::Act>
    ) -> impl Strategy1<G::Obs, G::Act, ()> {
        let g = self.g.borrow();



        let mut possible_states = g.visit_set();
        move |l| {
            todo!()
        }
    }*/
}

impl<G, const N: usize> MKBSC<G, N>
where
    G: Game<N>,
    G::Loc: Ord
{
    pub fn all_strategies(&self) -> AllStrategies<N> {
        AllStrategies::new(
            array_init(|i| self.kbsc[i].all_strategies1())
        )
    }

    /*pub fn translate_strategy(
        &self,
        strat: impl Fn(<Self as Game<N>>::Obs, G::Agt) -> G::Act
    ) -> impl FnMut(G::Obs, G::Agt) -> G::Act {
        
    }*/
}


pub trait Strategy<G: Game<N>, const N: usize> {
    type M;
    fn call(&self, obs: &G::Obs, mem: &Self::M, agt: G::Agt) -> Option<(G::Act, Self::M)>;
}

pub trait MemorylessStrategy<G: Game<N>, const N: usize>: Strategy<G, N, M=()> {
    fn call_ml(&self, obs: &G::Obs, agt: G::Agt) -> Option<G::Act> {
        self.call(obs, &(), agt).map(|(a, _)| a)
    }
}

pub trait Strategy1<G: Game1>: Strategy<G, 1> {
    fn call1(&self, obs: &G::Obs, mem: &Self::M) -> Option<(G::Act, Self::M)> {
        self.call(obs, mem, G::agent1())
    }
}

pub trait MemorylessStrategy1<G: Game1>:
    MemorylessStrategy<G, 1>
     + Strategy1<G, M=()>
{
    fn call_ml1(&self, obs: &G::Obs) -> Option<G::Act> {
        self.call_ml(obs, G::agent1())
    }
}

impl<T, G: Game<N>, const N: usize> MemorylessStrategy<G, N> for T
where T: Strategy<G, N, M=()> {}

impl<T, G: Game1> Strategy1<G> for T
where T: Strategy<G, 1> {}

impl<T, G: Game1> MemorylessStrategy1<G> for T
where T: MemorylessStrategy<G, 1> + Strategy1<G, M=()> {}

pub struct Strat<'a, F, Obs, Act, Agt, M>(F, PhantomData<&'a (Obs, Act, Agt, M)>)
where F: Fn(&Obs, &M, Agt) -> Option<(Act, M)> + 'a;

impl<'a, F, Obs, Act, Agt, M> Strat<'a, F, Obs, Act, Agt, M>
where F: Fn(&Obs, &M, Agt) -> Option<(Act, M)> + 'a {
    fn new(f: F) -> Self { Self(f, Default::default()) }
}

impl<'a, F, G, M, const N: usize> Strategy<G, N> for Strat<'a, F, G::Obs, G::Act, G::Agt, M>
where
    F: Fn(&G::Obs, &M, G::Agt) -> Option<(G::Act, M)>,
    G: Game<N>
{
    type M = M;
    fn call(&self, obs: &G::Obs, mem: &M, agt: G::Agt) -> Option<(G::Act, M)> {
        (self.0)(obs, mem, agt)
    }
}

pub fn strategy<'a, G: Game<N> + 'a, M: 'a, const N: usize>(f: impl Fn(&G::Obs, &M, G::Agt) -> Option<(G::Act, M)> + 'a) -> impl Strategy<G, N, M=M> + 'a {
    Strat::new(f)
}

pub fn strategy1<'a, G: Game1 + 'a, M: 'a>(f: impl Fn(&G::Obs, &M) -> Option<(G::Act, M)> + 'a) -> impl Strategy1<G, M=M> + 'a {
    strategy(move |obs, mem, _|
        f(obs, mem)
    )
}

pub fn memoryless_strategy<'a, G: Game<N> + 'a, const N: usize>(f: impl Fn(&G::Obs, G::Agt) -> Option<G::Act> + 'a) -> impl MemorylessStrategy<G, N, M=()> + 'a {
    strategy(move |obs, _, agt|
        f(obs, agt).map(|a| (a, ()))
    )
}

pub fn memoryless_strategy1<'a, G: Game1 + 'a>(f: impl Fn(&G::Obs) -> Option<G::Act> + 'a) -> impl MemorylessStrategy1<G, M=()> + 'a {
    memoryless_strategy(move |obs, _|
        f(obs)
    )
}
