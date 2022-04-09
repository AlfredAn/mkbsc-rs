use std::borrow::Cow;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::borrow::Borrow;
use crate::algo::KBSC;
use crate::algo::MKBSC;
use crate::game::dgame::DGame;
use crate::game::*;
use array_init::array_init;
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

    pub fn get(&self) -> impl MemorylessStrategy<DGame<N>, N> + '_ {
        memoryless_strategy(|obs, agt: AgtIndex|
            self.parts[agt.index()].get().call_ml1(obs)
        )
    }

    pub fn get_raw(&self) -> [&Vec<Option<ActionIndex>>; N] {
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

pub fn all_strategies<const N: usize>(g: [&DGame<1>; N]) -> AllStrategies<N> {
    AllStrategies::new(
        g.map(|g| g.all_strategies1())
    )
}

impl<R> KBSC<DGame<1>, R>
where
    R: Borrow<DGame<1>>
{
    pub fn translate_strategy<'b>(
        &'b self,
        strat: impl MemorylessStrategy1<Self> + 'b
    ) -> impl Strategy1<DGame<1>> + 'b
    {
        let g = self.g.borrow();
        
        strategy1(
            move |&obs, s: &Option<BTreeSet<_>>| {
                let s = if let Some(s) = s {
                    Cow::Borrowed(s)
                } else {
                    Cow::Owned(g.obs_set1(obs).iter().copied().collect())
                };
                let s = s.borrow();

                if let Some(a) = strat.call_ml1(s) {
                    let mut post = self.post1(s, a).dedup();
                    let s2 = post.next();
                    assert!(post.next().is_none()); // should be guaranteed by theory

                    s2.map(|s2| (a, s2))
                } else {
                    None
                }
            }
        )
    }
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

struct Strat<'a, F, G: Game<N>, M: Clone, const N: usize>(F, PhantomData<&'a (G, M)>)
where F: Fn(&G::Obs, &M, G::Agt) -> Option<(G::Act, M)> + 'a;

impl<'a, F, G: Game<N>, M: Clone, const N: usize> Strat<'a, F, G, M, N>
where F: Fn(&G::Obs, &M, G::Agt) -> Option<(G::Act, M)> + 'a {
    fn new(f: F) -> Self { Self(f, Default::default()) }
}

impl<'a, F, G, M, const N: usize> Strategy<G, N> for Strat<'a, F, G, M, N>
where
    F: Fn(&G::Obs, &M, G::Agt) -> Option<(G::Act, M)>,
    G: Game<N>,
    M: Clone
{
    type M = M;
    fn call(&self, obs: &G::Obs, mem: &M, agt: G::Agt) -> Option<(G::Act, M)> {
        (self.0)(obs, mem, agt)
    }
}

pub fn strategy<'a, G: Game<N> + 'a, M: Clone + 'a, const N: usize>(
    f: impl Fn(&G::Obs, &Option<M>, G::Agt) -> Option<(G::Act, M)> + 'a
) -> impl Strategy<G, N> + 'a {
        Strat::new(move |obs, mem, agt|
            f(obs, mem, agt).map(|(a, mem)|
                (a, Some(mem))
            )
        )
    }

pub fn strategy1<'a, G: Game1 + 'a, M: Clone + 'a>(
    f: impl Fn(&G::Obs, &Option<M>) -> Option<(G::Act, M)> + 'a
) -> impl Strategy1<G> + 'a {
    strategy(move |obs, mem, _|
        f(obs, mem)
    )
}

pub fn memoryless_strategy<'a, G: Game<N> + 'a, const N: usize>(
    f: impl Fn(&G::Obs, G::Agt) -> Option<G::Act> + 'a
) -> impl MemorylessStrategy<G, N> + 'a {
    Strat::new(move |obs, _, agt|
        f(obs, agt).map(|a| (a, ()))
    )
}

pub fn memoryless_strategy1<'a, G: Game1 + 'a>(
    f: impl Fn(&G::Obs) -> Option<G::Act> + 'a
) -> impl MemorylessStrategy1<G> + 'a {
    memoryless_strategy(move |obs, _|
        f(obs)
    )
}
