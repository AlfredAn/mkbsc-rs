pub mod strategy;
pub mod strategy1;
pub mod transducer;
pub mod translate;

use crate::*;
use thiserror::Error;

use StrategyError::*;
use Node::*;
use StackEntry::*;

pub use strategy::*;
pub use strategy1::*;
pub use transducer::*;
pub use translate::*;

#[derive(Error, Debug, Clone)]
pub enum StrategyError<M> {
    #[error("Strategy is not winning.")]
    Losing,
    #[error("Strategy is incomplete. ({0}, {1})")]
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

pub fn verify_strategy<S: Strategy, const N: usize>(
    g: &Game<N>,
    strat: &[S; N]
) -> Result<(), StrategyError<[S::M; N]>> {
    let mut stack = vec![Visit(
        g.l0(),
        array_init(|i| strat[i].init())
    )];
    let mut visit = HashMap::new();

    while let Some(entry) = stack.pop() {
        // println!("{:?}", entry);
        match entry {
            Visit(l, m) => {
                let key = (l, m);
                let m = &key.1;

                // println!("  visit: {:?}", l);

                match visit.get(&key) {
                    None => {
                        if g.is_winning(l) {
                            let r = visit.insert(key, Black);
                            assert_eq!(r, None);
                            continue;
                        }

                        let obs = g.observe(l);
                        if let Some(x) = from_iter(
                            (0..N).map_while(
                                |i| {
                                    // println!("    {:?}:", i);
                                    // println!("      strat({:?})", obs[i]);
                                    let x = strat[i].call(obs[i], &m[i]);
                                    // println!("      a={:?}", x.as_ref().map(|x| x.0));
                                    x
                                }
                            )
                        ) {
                            let a = array_init(|i| x[i].0);
                            let m2 = x.map(|(_, m)| m);
                            
                            // println!("    pushing {l:?}:{a:?}");
                            let mut at_least_one = false;
                            for l2 in g.post(l, a) {
                                if !at_least_one {
                                    stack.push(Finish(l, m.clone()));
                                    at_least_one = true;
                                }
                                // println!("      post: {:?}", l2);
                                stack.push(Visit(l2, m2.clone()))
                            }
                            if !at_least_one {
                                return Err(Incomplete(l, key.1));
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
