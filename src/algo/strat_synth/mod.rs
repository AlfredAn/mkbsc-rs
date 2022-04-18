pub mod strategy;
pub mod strategy1;
pub mod transducer;

use crate::*;
use thiserror::Error;

use StrategyError::*;
use Node::*;
use StackEntry::*;

pub use strategy::*;
pub use strategy1::*;
pub use transducer::*;

#[derive(Error, Debug, Clone)]
pub enum StrategyError<T, M> {
    #[error("Strategy is not winning.")]
    Losing,
    #[error("Strategy is incomplete. ({0}, {1})")]
    Incomplete(Loc<T>, M)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Gray,
    Black
}

#[derive(Debug, Clone)]
enum StackEntry<T, M> {
    Visit(Loc<T>, M),
    Finish(Loc<T>, M)
}

pub fn verify_strategy<T, S: StrategyProfile<T, N>, const N: usize>(
    g: &Game<T, N>,
    strat: &S
) -> Result<(), StrategyError<T, [S::M; N]>> where S::M: Debug {
    let mut stack = vec![Visit(g.l0(), strat.init())];
    let mut visit = HashMap::new();

    while let Some(entry) = stack.pop() {
        match entry {
            Visit(l, m) => {
                let key = (l, m);
                let m = &key.1;

                //println!("  visit: {:?}", key);

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
                                    //println!("    {:?}: {:?}", i, m);
                                    //println!("      strat({:?}, {:?})", obs[i], &m[i]);
                                    let x = strat.call(i, obs[i], &m[i]);
                                    //println!("      (a, m2)={:?}", x);
                                    x
                                }
                            )
                        ) {
                            let a = array_init(|i| x[i].0);
                            let m2 = x.map(|(_, m)| m);
                            
                            //println!("    pushing {:?}", l);
                            stack.push(Finish(l, m.clone()));
                            for l2 in g.post(l, a) {
                                //println!("      post: {:?}", l2);
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
