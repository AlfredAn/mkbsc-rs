pub mod strategy;
pub mod strategy1;
pub mod transducer;
pub mod translate;

use crate::*;

use Node::*;
use StackEntry::*;

pub use strategy::*;
pub use strategy1::*;
pub use transducer::*;
pub use translate::*;

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
) -> bool {
    let mut stack = vec![Visit(
        g.l0(),
        array_init(|i| strat[i].init())
    )];
    let mut visit = FxHashMap::default();

    while let Some(entry) = stack.pop() {
        match entry {
            Visit(l, m) => {
                let key = (l, m);
                let m = &key.1;

                match visit.get(&key) {
                    None => {
                        if g.is_winning(l) {
                            let r = visit.insert(key, Black);
                            assert_eq!(r, None);
                            continue;
                        }

                        let obs = g.observe(l);
                        if let Some(x) = from_iter::<_, _, N>(
                            (0..N).map_while(
                                |i| {
                                    let x = strat[i].call(obs[i], &m[i]);
                                    x
                                }
                            )
                        ) {
                            let a = array_init(|i| x[i].0);
                            let m2 = x.map(|(_, m)| m);
                            
                            let mut at_least_one = false;
                            for l2 in g.post(l, a) {
                                if !at_least_one {
                                    stack.push(Finish(l, m.clone()));
                                    at_least_one = true;
                                }
                                stack.push(Visit(l2, m2.clone()))
                            }
                            if !at_least_one {
                                return false;
                            }

                            let r = visit.insert(key, Gray);
                            assert_eq!(r, None);
                        } else {
                            return false;
                        }
                    },
                    Some(Gray) => {
                        return false;
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

    true
}
