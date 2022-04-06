pub mod strategy;
pub mod strategy1;

use crate::algo::SimAction;
use crate::algo::simulate;
use array_init::from_iter;
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

pub fn verify_strategy<'a, G, const N: usize>(
    g: &G,
    strat: impl Fn(&G::Obs, G::Agent) -> Option<G::Act>
) -> Result<(), StrategyError>
where
    G: Game<'a, N> + HasVisitSet<'a, N>
{
    simulate(g, (), |l, _, repeat|
        if repeat {
            SimAction::Stop(StrategyError::Losing)
        } else if g.is_winning(l) {
            SimAction::Skip
        } else {
            let obs = g.observe(l);
            if let Some(a) = from_iter(
                (0..N).map_while(
                    |i| strat(&obs[i], G::agent(i))
                )
            ) {
                SimAction::Visit(a, ())
            } else {
                SimAction::Stop(StrategyError::Incomplete)
            }
        }
    )
}

/*pub fn verify_strategy1<'a, G>(g: &G, strat: impl Fn(&G::Obs) -> Option<G::Act>) -> Result<(), StrategyError>
where
    G: Game1<'a> + HasVisitSet<'a, 1>
{
    verify_strategy(g, |obs, _| strat(obs))
}*/
