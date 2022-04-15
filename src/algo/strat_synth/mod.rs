pub mod strategy;
pub mod strategy1;

use crate::*;
use thiserror::Error;

use StrategyError::*;
use Node::*;
use StackEntry::*;

pub use strategy::*;
pub use strategy1::*;

#[derive(Error, Debug, Clone)]
pub enum StrategyError<M> {
    #[error("Strategy is not winning")]
    Losing,
    #[error("Strategy is incomplete")]
    Incomplete(Loc, M)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Gray,
    Black
}

#[derive(Debug, Clone)]
enum StackEntry<M> {
    Visit(Loc, M),
    Finish(Loc, M)
}

pub fn verify_strategy<T, S: Strategy<N>, const N: usize>(
    g: &Game<T, N>,
    strat: S
) -> Result<(), StrategyError<[S::M; N]>> {
    let mut stack = vec![Visit(g.l0(), strat.init())];
    let mut visit = HashMap::new();

    while let Some(entry) = stack.pop() {
        match entry {
            Visit(l, m) => {
                let key = (l, m);
                let m = &key.1;

                match visit.get(&key) {
                    None => {
                        let obs = g.observe(l);
                        if let Some(x) = from_iter(
                            (0..N).map_while(
                                |i| strat.call(obs[i], &m[i], i)
                            )
                        ) {
                            let a = array_init(|i| x[i].0);
                            let m2 = x.map(|(_, m)| m);
                            
                            stack.push(Finish(l, m.clone()));
                            for l2 in g.post(l, a) {
                                stack.push(Visit(l2, m2.clone()))
                            }

                            let r = visit.insert(key, Gray);
                            assert_eq!(r, None);
                        } else {
                            return Err(Incomplete(l, key.1));
                        }
                    },
                    Some(Gray) => {
                        return Err(Losing);
                    },
                    Some(Black) => ()
                }
            },
            Finish(l, m) => {
                let r = visit.insert((l, m), Black);
                assert_eq!(r, Some(Gray));
            }
        }
    }

    Ok(())
}
