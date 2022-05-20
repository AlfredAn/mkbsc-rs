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

    // eprintln!("--------------------------------------------------");

    // println!("{:?}", g);

    while let Some(entry) = stack.pop() {
        // eprintln!("pop: {:?}", entry);
        match entry {
            Visit(l, m) => {
                let key = (l, m);
                let m = &key.1;

                // eprintln!("  visit: {:?}", (l, m));
                // eprintln!("    value: {:?}", visit.get(&key));

                match visit.get(&key) {
                    None => {
                        if g.is_winning(l) {
                            // eprintln!("      winning");
                            let r = visit.insert(key, Black);
                            assert_eq!(r, None);
                            continue;
                        }

                        let a = (0..N).map(|i|
                            strat[i].action(&m[i])
                        ).collect_array()
                            .unwrap();
                        
                        // eprintln!("      a: {:?}", a);

                        let mut at_least_one = false;
                        for l2 in g.post(l, a) {
                            if g.is_winning(l2) {
                                stack.push(Finish(l, m.clone()));
                                at_least_one = true;
                                continue;
                            }

                            let o2 = g.observe(l2);
                            // eprintln!("        post: {:?} (obs={:?})", l2, o2);

                            if let Some(m2) = (0..N).map_while(|i| {
                                let r = strat[i].update(o2[i], &m[i]);
                                // eprintln!("          s{:?} -> ({}) -> {:?}", &m[i], display(|f| g.fmt_obs(f, agt(i), o2[i])), r);
                                r
                            }).collect_array::<N>() {
                                if !at_least_one {
                                    // eprintln!("          push: {:?}", Finish(l, m.clone()));
                                    stack.push(Finish(l, m.clone()));
                                    at_least_one = true;
                                }
                                // eprintln!("          push: {:?}", Visit(l2, m2.clone()));
                                stack.push(Visit(l2, m2.clone()))
                            } else {
                                // eprintln!("          no push");
                            }
                        }
                        if !at_least_one {
                            return false;
                        }

                        let r = visit.insert(key, Gray);
                        assert_eq!(r, None);
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
