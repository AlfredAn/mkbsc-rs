use std::borrow::Borrow;
use crate::algo::KBSC;
use std::marker::PhantomData;
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
