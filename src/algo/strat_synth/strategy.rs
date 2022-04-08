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
        g.map(|g| g.all_strategies())
    )
}

impl<'a, G, R> KBSC<'a, G, R>
where
    G: Game1<'a> + HasVisitSet<'a, 1>,
    G::Loc: Ord,
    R: Borrow<G>
{
    /*pub fn translate_strategy(
        &self,
        strat: impl Fn(<Self as Game<'a, 1>>::Loc) -> Option<G::Act>
    ) -> impl FnMut(G::Obs) -> Option<G::Act> + Clone {
        let g = self.g.borrow();



        let mut possible_states = g.visit_set();
        move |l| {
            todo!()
        }
    }*/
}

impl<'a, G, const N: usize> MKBSC<'a, G, N>
where
    G: Game<'a, N> + 'a,
    G::Loc: Ord
{
    pub fn all_strategies(&self) -> AllStrategies<N> {
        AllStrategies::new(
            array_init(|i| self.kbsc[i].dgame().all_strategies())
        )
    }

    /*pub fn translate_strategy(
        &self,
        strat: impl Fn(<Self as Game<'a, N>>::Obs, G::Agent) -> G::Act
    ) -> impl FnMut(G::Obs, G::Agent) -> G::Act {
        
    }*/
}


pub trait Strategy<Obs, Act, Agt, M> {
    fn call(&self, obs: &Obs, mem: &M, agt: Agt) -> Option<(Act, M)>;
}

pub trait MemorylessStrategy<Obs, Act, Agt>: Strategy<Obs, Act, Agt, ()> {
    fn call_ml(&self, obs: &Obs, agt: Agt) -> Option<Act> {
        self.call(obs, &(), agt).map(|(a, _)| a)
    }
}

pub trait Strategy1<Obs, Act, M>: Strategy<Obs, Act, ZeroIndex, M> {
    fn call1(&self, obs: &Obs, mem: &M) -> Option<(Act, M)> {
        self.call(obs, mem, ().into())
    }
}

pub trait MemorylessStrategy1<Obs, Act>:
    MemorylessStrategy<Obs, Act, ZeroIndex>
     + Strategy1<Obs, Act, ()>
{
    fn call_ml1(&self, obs: &Obs) -> Option<Act> {
        self.call_ml(obs, ().into())
    }
}

impl<T, Obs, Act, Agt> MemorylessStrategy<Obs, Act, Agt> for T
where T: Strategy<Obs, Act, Agt, ()> {}

impl<T, Obs, Act, M> Strategy1<Obs, Act, M> for T
where T: Strategy<Obs, Act, ZeroIndex, M> {}

impl<T, Obs, Act> MemorylessStrategy1<Obs, Act> for T
where T: MemorylessStrategy<Obs, Act, ZeroIndex> + Strategy1<Obs, Act, ()> {}

pub struct Strat<'a, F, Obs, Act, Agt, M>(F, PhantomData<&'a (Obs, Act, Agt, M)>)
where F: Fn(&Obs, &M, Agt) -> Option<(Act, M)> + 'a;

impl<'a, F, Obs, Act, Agt, M> Strat<'a, F, Obs, Act, Agt, M>
where F: Fn(&Obs, &M, Agt) -> Option<(Act, M)> + 'a {
    fn new(f: F) -> Self { Self(f, Default::default()) }
}

impl<'a, F, Obs, Act, Agt, M> Strategy<Obs, Act, Agt, M> for Strat<'a, F, Obs, Act, Agt, M>
where F: Fn(&Obs, &M, Agt) -> Option<(Act, M)> {
    fn call(&self, obs: &Obs, mem: &M, agt: Agt) -> Option<(Act, M)> {
        (self.0)(obs, mem, agt)
    }
}

pub fn strategy<'a, Obs: 'a, Act: 'a, Agt: 'a, M: 'a>(f: impl Fn(&Obs, &M, Agt) -> Option<(Act, M)> + 'a) -> impl Strategy<Obs, Act, Agt, M> + 'a {
    Strat::new(f)
}

pub fn strategy1<'a, Obs: 'a, Act: 'a, M: 'a>(f: impl Fn(&Obs, &M) -> Option<(Act, M)> + 'a) -> impl Strategy1<Obs, Act, M> + 'a {
    strategy(move |obs, mem, _|
        f(obs, mem)
    )
}

pub fn memoryless_strategy<'a, Obs: 'a, Act: 'a, Agt: 'a>(f: impl Fn(&Obs, Agt) -> Option<Act> + 'a) -> impl MemorylessStrategy<Obs, Act, Agt> + 'a {
    strategy(move |obs, _, agt|
        f(obs, agt).map(|a| (a, ()))
    )
}

pub fn memoryless_strategy1<'a, Obs: 'a, Act: 'a>(f: impl Fn(&Obs) -> Option<Act> + 'a) -> impl MemorylessStrategy1<Obs, Act> + 'a {
    memoryless_strategy(move |obs, _|
        f(obs)
    )
}
