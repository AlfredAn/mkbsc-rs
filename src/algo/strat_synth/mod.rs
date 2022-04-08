pub mod strategy;
pub mod strategy1;

use std::marker::PhantomData;
use crate::algo::SimAction;
use crate::algo::simulate;
use array_init::*;
use crate::*;

use thiserror::Error;

pub use strategy::*;
pub use strategy1::*;

#[derive(Error, Debug, Clone)]
pub enum StrategyError {
    #[error("Strategy is not winning")]
    Losing,
    #[error("Strategy is incomplete")]
    Incomplete
}

pub fn verify_strategy<'a, G, M, const N: usize>(
    g: &G,
    init: [M; N],
    strat: impl Strategy<G::Obs, G::Act, G::Agent, M>
) -> Result<(), StrategyError>
where
    G: Game<'a, N> + HasVisitSet<'a, N>,
    M: Clone
{
    simulate(g, init, |l, mem, is_visited|
        if g.is_winning(l) {
            SimAction::Skip
        } else if is_visited {
            SimAction::Stop(StrategyError::Losing)
        } else {
            let obs = g.observe(l);
            if let Some(x) = from_iter(
                (0..N).map_while(
                    |i| strat.call(&obs[i], &mem[i], G::agent(i))
                )
            ) {
                let a = array_init(|i| x[i].0);
                let mem = x.map(|(_, m)| m);
                SimAction::Visit(a, mem)
            } else {
                SimAction::Stop(StrategyError::Incomplete)
            }
        }
    )
}

pub fn verify_memoryless_strategy<'a, G, const N: usize>(
    g: &G,
    strat: impl MemorylessStrategy<G::Obs, G::Act, G::Agent>
) -> Result<(), StrategyError>
where
    G: Game<'a, N> + HasVisitSet<'a, N>
{
    verify_strategy(g, [(); N], strat)
}
