
use derive_more::{IsVariant, Add, AddAssign};
use vec_map::VecMap;

use crate::*;
use Node::*;
use StackEntry::*;
use BacktrackEntry::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Node {
    Gray,
    Black
}

#[derive(Debug, Copy, Clone, IsVariant)]
enum StackEntry {
    Visit(Loc),
    Finish(Loc),
    Backtrack
}

#[derive(Debug, Copy, Clone)]
enum BacktrackEntry<const N: usize> {
    StackPop(StackEntry),
    StackPush(u32),
    VisitInsert(Loc, Option<Node>),
    StratInsert(Loc, [bool; N]),

    Branch(Loc, [Act; N])
}

#[derive(Debug, Copy, Clone, Default, Add, AddAssign)]
pub struct Stats {
    steps: u64,
    nodes: u64,
    backtracks: i64
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Strat(VecMap<Act>, PtrEqRc<Game<1>>);

impl Debug for Strat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_set(f, self.0.iter(), |f, (l, &a)| {
            write!(f, "({l}){}:{}", display(|f| self.1.fmt_loc(f, loc(l))), a)
        })
    }
}

impl MemorylessStrategy for Strat {
    fn call_ml(&self, obs: Obs) -> Option<Act> {
        if let Some(l) = self.1.to_unique_loc(obs, agt(0)) {
            self.0.get(l.index()).copied()
        } else {
            panic!("expected a perfect information game")
        }
    }

    fn init_ml(&self) -> Act {
        *self.0.get(self.1.l0().index()).unwrap()
    }
}

pub fn find_strategy<const N: usize>(
    g: &ConstructedGame<MKBSC<N>, N>,
    mut new_depth: impl FnMut(u32) -> ControlFlow<()>,
    mut found_strat: impl FnMut(&[Strat; N]) -> ControlFlow<()>,
    find_all: bool
) -> Stats {
    let mut stats = Stats::default();

    for max_depth in 0.. {
        if new_depth(max_depth).is_break() { break; }

        let mut brk = false;
        let hit_ceiling = _find_strategy(
            g.game.clone(),
            array_init(|i| g.origin().gki[i].game.clone()),
            max_depth,
            |agt, l_gk| {
                // println!("\n+agt={agt}");
                // println!("+l_gk={l_gk}, {}", display(|f| g.fmt_loc(f, l_gk)));
                let l_gki = g.origin_loc(l_gk)[agt.index()];
                // println!("+l_gki={l_gki}, {}\n", display(|f| g.origin().gki[agt.index()].fmt_loc(f, l_gki)));
                l_gki
            },
            |strat| {
                let r = found_strat(strat);
                // println!("{:?}", r);
                if r.is_break() {
                    brk = true;
                }
                r
            },
            &mut stats,
            find_all
        );
        // println!("{:?}", stats);

        if brk || !hit_ceiling { break; }
    }

    stats
}

fn _find_strategy<const N: usize>(
    gk: Rc<Game<N>>,
    gki: [Rc<Game<1>>; N],
    max_depth: u32,
    mut to_l_gki: impl FnMut(Agt, Loc) -> Loc,
    mut f: impl FnMut(&[Strat; N]) -> ControlFlow<()>,
    stats: &mut Stats,
    find_all: bool
) -> bool {
    let outcomes: [_; N] = array_init(|i| find_outcomes(&*gki[i]));
    // println!("{:#?}", outcomes);

    let mut visit = VecMap::with_capacity(gk.n_loc());

    let mut strat: [_; N] = array_init(|i|
        Strat(VecMap::with_capacity(gki[i].n_loc()), PtrEqRc(gki[i].clone()))
    );

    let mut depth = 0u32;

    let mut stack = vec![Visit(gk.l0())];
    let mut backtrack = vec![StackPush(1)];

    let mut options: [_; N] = array_init(|_| vec![]);

    let mut hit_ceiling = false;

    'outer: loop {
        // // println!("                                                  {:?}", stack);
        // // println!("                                                  {:?}", backtrack);

        if let Some(entry) = stack.pop() {
            stats.steps += 1;
            // println!("  pop: {:?}", entry);

            if !entry.is_backtrack() {
                backtrack.push(StackPop(entry));
            }

            match entry {
                Backtrack => {
                    stats.backtracks += 1;
                    loop {
                        stats.steps += 1;

                        let bt = backtrack.pop();
                        // println!("  backtrack: {:?}", bt);
                        match bt {
                            Some(StackPop(entry)) => stack.push(entry),
                            Some(StackPush(count)) => {
                                assert!(stack.len() >= count as usize);
                                for _ in 0..count {
                                    stack.pop();
                                }
                            },
                            Some(VisitInsert(l, old)) => if let Some(old) = old {
                                visit.insert(l.index(), old);
                            } else {
                                visit.remove(l.index());
                            },
                            Some(StratInsert(l, changed)) => {
                                for agt in gk.iter_agt() {
                                    if changed[agt.index()] {
                                        strat[agt.index()].0.remove(to_l_gki(agt, l).index());
                                    }
                                }
                                assert!(depth != 0);
                                depth -= 1;
                            },
                            Some(Branch(l, a)) => {
                                let mut changed = ArrayVec::new();
                                for agt in gk.iter_agt() {
                                    changed.push(
                                        strat[agt.index()].0.insert(
                                            to_l_gki(agt, l).index(),
                                            a[agt.index()]
                                        ).is_none()
                                    );
                                }
                                depth += 1;

                                backtrack.push(
                                    StratInsert(
                                        l,
                                        changed.into_inner().unwrap()
                                    )
                                );

                                let mut count = 1;
                                stack.push(Finish(l));
                                for l2 in gk.post(l, a) {
                                    stack.push(Visit(l2));
                                    count += 1;
                                }
                                backtrack.push(StackPush(count));
                                break;
                            },
                            None => return hit_ceiling,
                        }
                    }
                },
                Visit(l) => {
                    match visit.get(l.index()) {
                        None => {
                            if gk.is_winning(l) {
                                // println!("    winning state -> skipping");
                                let old = visit.insert(l.index(), Black);
                                backtrack.push(VisitInsert(l, old));
                                assert!(old.is_none());
                                continue;
                            }

                            if depth >= max_depth {
                                // println!("    hit ceiling -> backtracking");
                                stack.push(Backtrack);
                                hit_ceiling = true;
                                continue;
                            }

                            stats.nodes += 1;

                            let l_gki: [_; N] = array_init(|i| to_l_gki(agt(i), l));
                            for agt in gk.iter_agt() {
                                let l_gki = l_gki[agt.index()];
                                let outcome = &outcomes[agt.index()][l_gki.index()];

                                options[agt.index()].clear();

                                match (find_all, outcome.outcome) {
                                    (false, Outcome::Win(_)) => options[agt.index()].push({
                                        let r = gk.iter_act(agt)
                                            .map(|a| (a, outcome.action[a.index()]))
                                            .min_by_key(|&(_, o)| o)
                                            .unwrap();
                                        assert!(r.1.is_win());
                                        r
                                    }),
                                    (_, Outcome::Maybe(_, _)) | (true, Outcome::Win(_)) => {
                                        options[agt.index()].extend(
                                            gk.iter_act(agt)
                                                .map(|a| (a, outcome.action[a.index()]))
                                                .filter(|(_, o)| o.can_win())
                                            );
                                        options[agt.index()].sort_unstable_by_key(|&(_, o)| o);
                                    },
                                    (_, Outcome::Lose) => {
                                        // println!("    all actions losing -> backtracking");
                                        stack.push(Backtrack);
                                        continue 'outer;
                                    },
                                }
                            }

                            let old = visit.insert(l.index(), Gray);
                            backtrack.push(VisitInsert(l, old));
                            assert_eq!(old, None);

                            let mut at_least_one = false;
                            cartesian_product(array_init(|i| &*options[i]), |x| {
                                let a = x.map(|&(a, _)| a);

                                if gk.post(l, a).next().is_none() {
                                    return;
                                }

                                at_least_one = true;

                                if (0..N).all(|i|
                                    if let Some(&a_) = strat[i].0.get(l_gki[i].index()) {
                                        a[i] == a_
                                    } else {
                                        true
                                    }
                                ) {
                                    // println!("      pushing branch: {a:?}");
                                    backtrack.push(Branch(l, a));
                                }
                            });
                            if at_least_one {
                                stats.backtracks -= 1;
                            }
                            // println!("    branching...");
                            stack.push(Backtrack);
                        },
                        Some(Gray) => {
                            // println!("    cycle detected -> backtracking");
                            stack.push(Backtrack)
                        },
                        Some(Black) => {
                            // println!("    already visited -> skipping");
                        }
                    }
                },
                Finish(l) => {
                    let old = visit.insert(l.index(), Black);
                    backtrack.push(VisitInsert(l, old));
                    assert_eq!(old, Some(Gray));
                }
            }
        } else {
            // println!("    strategy found");
            if f(&strat).is_break() {
                return hit_ceiling;
            }
            // println!("    ...backtracking");
            stack.push(Backtrack);
        }
    }
}

pub trait Strategy {
    type M: Clone + Eq + Hash;

    fn update(&self, obs: Obs, mem: &Self::M) -> Option<Self::M>;
    fn action(&self, mem: &Self::M) -> Act;

    fn init(&self) -> Self::M;

    fn init_tuple(&self) -> (Act, Self::M) {
        let mem = self.init();
        let act = self.action(&mem);
        (act, mem)
    }

    fn call(&self, obs: Obs, mem: &Self::M) -> Option<(Act, Self::M)> {
        self.update(obs, mem).map(|mem|
            (self.action(&mem), mem)
        )
    }

    fn transducer(&self, g: &Game<1>) -> Transducer {
        Transducer::build(g, self)
    }

    fn transducer_ma<const N: usize>(&self, g: &Game<N>, agt: Agt) -> Transducer {
        Transducer::build_ma(g, agt, self)
    }
}

impl<S: MemorylessStrategy> Strategy for S {
    type M = Act;

    fn update(&self, obs: Obs, _: &Self::M) -> Option<Self::M> {
        self.call_ml(obs)
    }

    fn action(&self, &mem: &Self::M) -> Act {
        mem
    }

    fn init(&self) -> Self::M {
        self.init_ml()
    }
}

pub trait MemorylessStrategy {
    fn call_ml(&self, obs: Obs) -> Option<Act>;
    fn init_ml(&self) -> Act;
}

struct FnStrat<M, U, A>(M, U, A);

impl<
    M: Clone + Eq + Hash,
    U: Fn(Obs, &M) -> Option<M>,
    A: Fn(&M) -> Act
> Strategy for FnStrat<M, U, A> {
    type M = M;

    fn update(&self, obs: Obs, mem: &Self::M) -> Option<Self::M> {
        self.1(obs, mem)
    }

    fn action(&self, mem: &Self::M) -> Act {
        self.2(mem)
    }

    fn init(&self) -> Self::M {
        self.0.clone()
    }
}

struct FnStratAlt<M, F>((Act, M), F);

impl<
    M: Clone + Eq + Hash,
    F: Fn(Obs, &M) -> Option<(Act, M)>
> Strategy for FnStratAlt<M, F> {
    type M = (Act, M);

    fn update(&self, obs: Obs, (_, mem): &Self::M) -> Option<Self::M> {
        self.1(obs, mem)
    }

    fn action(&self, (a, _): &Self::M) -> Act {
        *a
    }

    fn init(&self) -> Self::M {
        self.0.clone()
    }
}

struct MLessFnStrat<F>(Act, F);

impl<
    F: Fn(Obs) -> Option<Act>
> MemorylessStrategy for MLessFnStrat<F> {
    fn call_ml(&self, obs: Obs) -> Option<Act> {
        self.1(obs)
    }

    fn init_ml(&self) -> Act {
        self.0
    }
}

pub fn strategy<M: Clone + Eq + Hash>(
    init: M,
    u: impl Fn(Obs, &M) -> Option<M>,
    a: impl Fn(&M) -> Act
) -> impl Strategy {
    FnStrat(init, u, a)
}

pub fn strategy_alt<M: Clone + Eq + Hash>(
    init: (Act, M),
    f: impl Fn(Obs, &M) -> Option<(Act, M)>
) -> impl Strategy {
    FnStratAlt(init, f)
}

pub fn memoryless_strategy(
    g: &Game<1>,
    f: impl Fn(Obs) -> Option<Act>
) -> Option<impl MemorylessStrategy> {
    memoryless_strategy_ma(g, agt(0), f)
}

pub fn memoryless_strategy_ma<const N: usize>(
    g: &Game<N>,
    agt: Agt,
    f: impl Fn(Obs) -> Option<Act>
) -> Option<impl MemorylessStrategy> {
    f(g.obs0()[agt.index()]).map(|a|
        MLessFnStrat(a, f)
    )
}
