use std::collections::*;
use disjoint_sets::UnionFind;
use itertools::Itertools;
use paste::paste;
use anyhow::{bail, ensure, Context};
use array_init::{array_init};

use crate::*;

use super::*;

type Agt = Symbol;
type Act = Symbol;
type Loc = Symbol;
type Obs = crate::Obs;

macro_rules! io_game {
    ($($n:expr),*; $dol:tt) => {
        paste! {
            #[derive(Debug)]
            #[enum_dispatch(IOGameTrait)]
            pub enum IOGameEnum {
                $(
                    [<N $n>](IOGame<$n>)
                ),*
            }

            macro_rules! new {
                ($n_dyn:expr, $dol($arg:expr),*) => {{
                    let n_dyn = $n_dyn;
                    match n_dyn {
                        $(
                            $n => Ok(IOGameEnum::[<N $n>](IOGame::<$n>::new($dol($arg),*)?))
                        ),*,
                        _ => bail!("unsupported number of agents: {}", n_dyn)
                    }
                }}
            }

            impl IOGameEnum {
                // pub fn build() -> 
            }
        }
    };
}

io_game!(1, 2, 3, 4, 5, 6, 7, 8; $);

impl IOGameEnum {
    pub fn new(v: Vec<Statement>) -> anyhow::Result<Self> {
        let mut v2 = vec![];

        let mut agents = HashMap::new();
        let mut actions = HashMap::new();
        let mut locations = HashMap::new();
        let mut reach = HashSet::new();

        for st in v.into_iter() {
            match st {
                Statement::Agt(list) => {
                    for agt in list {
                        ensure!(!agents.contains_key(&agt), "duplicate agent: {}", &agt);
                        agents.insert(agt, crate::agt(agents.len()));
                    }
                },
                Statement::Act(list) => {
                    for act in list {
                        ensure!(!actions.contains_key(&act), "duplicate action: {}", &act);
                        actions.insert(act, crate::act(actions.len()));
                    }
                },
                Statement::Loc(list) => {
                    for l in list {
                        ensure!(!locations.contains_key(&l), "duplicate location: {}", &l);
                        locations.insert(l, crate::loc(locations.len()));
                    }
                },
                Statement::Reach(list) => {
                    for l in list {
                        if reach.contains(&l) {
                            println!("--WARNING-- duplicate entry in reach: {}", &l);
                        }
                        reach.insert(l);
                    }
                },
                x => v2.push(x)
            }
        }

        new!(agents.len(), agents, actions, locations, reach, v2)
    }
}
 
#[derive(Debug)]
pub struct IOGame<const N: usize> {
    agt: [Agt; N],
    act: Vec<Act>,
    loc: HashMap<Loc, crate::Loc>,
    obs: [UnionFind<u32>; N],
    delta: Vec<(Loc, [crate::Act; N], Loc)>,
    l0: Loc,
    reach: HashSet<Loc>
}

#[enum_dispatch]
pub trait IOGameTrait {
    fn n_agents(&self) -> usize;
}

impl<const N: usize> IOGameTrait for IOGame<N> {
    fn n_agents(&self) -> usize { N }
}

impl<const N: usize> IOGame<N> {
    fn new(
        agents: HashMap<Agt, crate::Agt>,
        act: HashMap<Act, crate::Act>,
        loc: HashMap<Loc, crate::Loc>,
        reach: HashSet<Loc>,
        v: Vec<Statement>
    ) -> anyhow::Result<IOGame<N>> {

        let get_loc = |name| loc.get(&name)
            .with_context(|| format!("undefined location: {}", &name));
        let get_act = |name| act.get(&name)
            .with_context(|| format!("undefined action: {}", &name));

        let mut obs: [_; N] = array_init(|_|
            UnionFind::<u32>::new(loc.len())
        );
        let mut l0 = None;
        let mut delta = BTreeSet::new();

        for st in v.into_iter() {
            match st {
                Statement::L0(l) => {
                    ensure!(l0.is_none(), "l0 defined multiple times");
                    ensure!(loc.contains_key(&l), "undefined location: {}", l);
                    l0 = Some(l);
                },
                Statement::Obs(agt, list) => {
                    let &agt = agents.get(&agt)
                        .with_context(|| format!("undefined agent: {}", agt))?;
                    for equivalence in list.into_iter() {
                        for (a, b) in equivalence.into_iter().tuple_windows() {
                            let (&a, &b) = (get_loc(a)?, get_loc(b)?);
                            obs[agt.index()].union(a.value(), b.value());
                        }
                    }
                },
                Statement::Delta(list) => {
                    for (l, a_, l2) in list.into_iter() {
                        ensure!(loc.contains_key(&l), "undefined location: {}", l);
                        ensure!(a_.len() == N, "expected {} actions, found {}", N, a_.len());

                        let mut a = ArrayVec::<_, N>::new();
                        for a_ in a_ {
                            a.push(*get_act(a_)?);
                        }
                        let a: [_; N] = (*a).try_into().unwrap();

                        ensure!(loc.contains_key(&l2), "undefined location: {}", l2);

                        let tr = (l, a, l2);
                        
                        if delta.contains(&tr) {
                            println!("--WARNING-- duplicate transition: {} ({}) {}", &tr.0, tr.1.iter().format(", "), &tr.2);
                        }
                        delta.insert(tr);
                    }
                },
                _ => unreachable!()
            }
        }

        let agt = from_iter(
            agents.into_iter()
                .sorted_by(|(_, i), (_, j)| i.cmp(j))
                .map(|(name, _)| name)
        ).unwrap();

        let act = act.into_iter()
            .sorted_by(|(_, i), (_, j)| i.cmp(j))
            .map(|(name, _)| name)
            .collect();
            
        let delta = delta.into_iter()
            .collect();

        let l0 = if let Some(l0) = l0 {
            l0
        } else {
            bail!("l0 is not defined")
        };

        Ok(
            Self {
                agt,
                act,
                loc,
                delta,
                obs,
                l0,
                reach
            }.into()
        )
    }
}

impl<const N: usize> IOGame<N> {
    pub fn find_agent(&self, name: &Symbol) -> Option<crate::Agt> {
        self.agt.iter()
            .enumerate()
            .find(|(_, agt)| *agt == name)
            .map(|(i, _)| agt(i))
    }
}

impl<const N: usize> AbstractGame<N> for IOGame<N> {
    type Loc = Loc;
    type Obs = Obs;

    fn l0(&self) -> Loc {
        self.l0.clone()
    }

    fn n_actions(&self) -> [usize; N] {
        [self.act.len(); N]
    }

    fn obs(&self, l: &Loc) -> [Obs; N] {
        let entry = self.loc[l].value();
        array_init(|i| {
            crate::obs(self.obs[i].find(entry))
        })
    }

    fn is_winning(&self, l: &Loc) -> bool {
        self.reach.contains(l)
    }

    fn succ(
        &self,
        l: &Loc,
        mut f: impl FnMut([crate::Act; N], Loc)
    ) {
        let found = find_group(&self.delta, |(l_, _, _)| l.cmp(l_));
        for (l_, a, l2) in found {
            assert!(l_ == l);
            f(*a, l2.clone());
        }
    }

    fn fmt_loc(&self, f: &mut std::fmt::Formatter, l: &Loc) -> std::fmt::Result {
        write!(f, "{}", l)
    }
}
