pub mod strategy;
pub mod strategy1;

use crate::algo::*;
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

pub fn verify_strategy<T, M: Clone, const N: usize>(
    g: &Game<T, N>,
    init: [M; N],
    strat: impl Strategy<N, M=M>
) -> Result<(), StrategyError> {
    simulate(g, init, |l, mem, is_visited|
        if g.is_winning(l) {
            SimAction::Skip
        } else if is_visited {
            SimAction::Stop(StrategyError::Losing)
        } else {
            let obs = g.observe(l);
            if let Some(x) = from_iter(
                (0..N).map_while(
                    |i| strat.call(obs[i], &mem[i], i)
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

pub fn verify_memoryless_strategy<T, const N: usize>(
    g: &Game<T, N>,
    strat: impl MemorylessStrategy<N>
) -> Result<(), StrategyError> {
    verify_strategy(g, [(); N], strat)
}
