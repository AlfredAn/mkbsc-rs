use std::collections::hash_map::Entry::*;
use std::ops::Range;
use petgraph::visit::EdgeRef;
use petgraph::graph::{NodeIndex, node_index};
use arrayvec::ArrayVec;
use std::collections::*;
use petgraph::{Graph, Directed};
use super::*;
use smart_default::SmartDefault;
use array_init::array_init;

pub type Loc = u32;
pub type Obs = u32;
pub type ObsOffset = u32;

type GraphType<T, const N: usize>
    = Graph<LocData<T, N>, [Act; N], Directed>;

#[derive(Debug, Clone, SmartDefault)]
pub struct Game<T, const N: usize> {
    pub graph: GraphType<T, N>,

    #[default([0; N])]
    pub n_actions: [usize; N],

    #[default(array_init(|_| Vec::new()))]
    pub obs: [Vec<Vec<Loc>>; N]
}

#[derive(Debug, Clone, Copy)]
pub struct LocData<T, const N: usize> {
    pub is_winning: bool,
    pub obs: [Obs; N],
    pub obs_offset: [ObsOffset; N],
    pub data: T
}

fn ni(l: Loc) -> NodeIndex<Loc> {
    node_index(l as usize)
}
fn li(ni: NodeIndex<Loc>) -> Loc {
    ni.index() as Loc
}

impl<T, const N: usize> Game<T, N> {
    pub fn n_loc(&self) -> usize {
        self.graph.node_count()
    }
    pub fn n_edges(&self) -> usize {
        self.graph.edge_count()
    }
    pub fn n_obs(&self, agt: Agt) -> usize {
        self.obs[agt as usize].len()
    }

    pub fn loc_data(&self, l: Loc) -> &LocData<T, N> {
        &self.graph[ni(l)]
    }

    pub fn observe(&self, l: Loc) -> [Obs; N] {
        self.loc_data(l).obs
    }
    pub fn obs_offset(&self, l: Loc) -> [ObsOffset; N] {
        self.loc_data(l).obs_offset
    }
    pub fn is_winning(&self, l: Loc) -> bool {
        self.loc_data(l).is_winning
    }
    pub fn data(&self, l: Loc) -> &T {
        &self.loc_data(l).data
    }

    pub fn obs_set(&self, agt: Agt, obs: Obs) -> &[Loc] {
        &*self.obs[agt][obs as usize]
    }

    pub fn succ<'a>(&'a self, l: Loc) -> impl Iterator<Item=([Act; N], Loc)> + 'a {
        self.graph.edges(ni(l))
            .map(|e| (*e.weight(), li(e.target())))
    }
}

impl<G: AbstractGame<N>, const N: usize> From<&G> for Game<G::Loc, N> {
    fn from(g: &G) -> Self {
        let mut r = Game::default();
        r.n_actions = g.n_actions();

        let mut stack = Vec::new();
        let mut visited = HashMap::new();
        let mut obs_map = HashMap::new();

        macro_rules! visit {
            ($l:expr) => {
                {
                    let l = $l;

                    let n = r.n_loc() as Loc;

                    let o = g.obs(&l);

                    let mut obs = ArrayVec::<_, N>::new();
                    let mut obs_offset = ArrayVec::<_, N>::new();
                    for (agt, oi) in o.into_iter().enumerate() {
                        let obs_i = match obs_map.entry((agt, oi)) {
                            Vacant(e) => {
                                let obs_i = r.obs[agt].len() as Obs;
                                r.obs[agt].push(Vec::new());
                                e.insert(obs_i);
                                obs_i
                            },
                            Occupied(e) => {
                                *e.get()
                            }
                        };
                        let obs_set = &mut r.obs[agt][obs_i as usize];

                        obs.push(obs_i);
                        obs_offset.push(obs_set.len() as ObsOffset);

                        obs_set.push(n);
                    }

                    stack.push(l.clone());
                    visited.insert(l.clone(), ni(n));

                    let n_ = r.graph.add_node(LocData {
                        is_winning: g.is_winning(&l),
                        obs: (*obs).try_into().unwrap(),
                        obs_offset: (*obs_offset).try_into().unwrap(),
                        data: l
                    });
                    assert_eq!(ni(n), n_);

                    n_
                }
            }
        }

        visit!(g.l0());

        let mut i = 0;
        while let Some(l) = stack.pop() {
            let n = ni(i as Loc);

            g.succ(&l, |a, l2| {
                let n2 = if let Some(&n2) = visited.get(&l2) {
                    n2
                } else {
                    visit!(l2)
                };
                if !r.graph.edges_connecting(n, n2).any(|e| *e.weight() == a) {
                    r.graph.add_edge(n, n2, a);
                }
            });

            i += 1;
        }

        r
    }
}
