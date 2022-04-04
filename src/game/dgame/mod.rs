use std::ops::Index;
use std::cell::RefCell;
use petgraph::Incoming;
use crate::game::*;
use std::{fmt, slice, iter};
use crate::macros::*;

use fixedbitset::FixedBitSet;
use petgraph::{visit::*, graph::{IndexType, Neighbors, node_index, EdgeReference, EdgeReferences}, Graph, Directed, Direction};
use array_init::array_init;
use itertools::Itertools;

use self::{index::*, edge::DEdge, obs::DObs, node::DNode};

use super::{Game, macros::{derive_ma, derive_ii, derive_magiian}};

use crate::{game::macros, util::*};

pub mod index;
pub mod node;
pub mod edge;
pub mod obs;
pub mod from_game;
pub mod builder;
pub mod generic_builder;

type GraphType<const N: usize>
    = Graph<DNode<N>, DEdge<N>, Directed>;

#[derive(Clone)]
pub struct DGame<const N: usize> {
    pub graph: GraphType<N>,
    pub l0: NodeIndex,
    pub n_actions: usize,
    pub obs: [Vec<DObs>; N]
}

impl<'a, const N: usize> Game<'a, N> for DGame<N> {
    type Loc = NodeIndex;
    type Act = ActionIndex;

    fn l0<'b>(&'b self) -> &'b Self::Loc where 'a: 'b {
        &self.l0
    }

    fn actions<'b>(&'b self) -> Itr<'b, [Self::Act; N]> where 'a: 'b {
        Box::new(range_power::<N>(0..self.n_actions).map(|x| array_init(|i| action_index(x[i]))))
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        self.node(*n).is_winning
    }

    fn post<'b>(&'b self, n: &'b Self::Loc, a: [Self::Act; N]) -> Itr<'b, Self::Loc> where 'a: 'b {
        Box::new(self.graph.edges(*n).filter(move |e| e.weight().act.contains(&a)).map(|e| e.target()))
    }

    type Obs = ObsIndex;

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; N] {
        self.node(*l).obs
    }

    type Agent = AgentIndex;

    fn actions_i<'b>(&'b self, _: Self::Agent) -> Itr<'b, Self::Act> where 'a: 'b {
        Box::new((0..self.n_actions).map(|a| action_index(a)))
    }

    fn debug_string(&self, l: &Self::Loc) -> Option<String> {
        self.node(*l).debug.clone()
    }

    fn dgame<'b>(&self) -> Cow<Self> where 'a: 'b {
        Cow::Borrowed(self)
    }

    fn into_dgame(self) -> Self {
        self
    }
}

impl<'a> Game1<'a> for DGame<1> {
    fn all_strategies(&self) -> AllStrategies1 {
        let w = find_memoryless_strategies(self);
        AllStrategies1::new(&w, self.n_actions)
    }
}

impl DGame<1> {
    pub fn pre<'b>(&'b self, s: impl IntoIterator<Item=NodeIndex> + 'b, a: ActionIndex) -> impl Iterator<Item=NodeIndex> + 'b {
        s.into_iter()
            .map(move |n| self.graph.edges_directed(n, Incoming)
                .filter(move |e| e.weight().act.contains(&[a]))
                .map(|e| e.source())
            ).flatten()
    }

    pub fn pre_all<'b>(&'b self, i: impl IntoIterator<Item=NodeIndex> + Clone + 'b) -> impl Iterator<Item=(NodeIndex, ActionIndex)> + 'b {
        self.actions1()
            .flat_map(move |a|
                self.pre(i.clone(), a)
                    .map(move |l| (l, a))
            )
    }

    pub fn cpre<'b>(
        &'b self,
        mut s: impl FnMut(usize) -> bool + 'b,
        i: impl Iterator<Item=NodeIndex> + Clone + 'b
    ) -> impl Iterator<Item=(NodeIndex, ActionIndex)> + 'b
    {
        self.pre_all(i)
            .filter(move |(l, a)|
                self.post1(l, *a)
                    .all(|l2| s(l2.index()))
            )
    }

    pub fn pre_winnable<'b>(
        &'b self,
        mut s: impl FnMut(usize) -> bool + 'b,
        i: impl Iterator<Item=NodeIndex> + Clone + 'b
    ) -> impl Iterator<Item=(NodeIndex, ActionIndex)> + 'b
    {
        self.pre_all(i)
            .filter(move |(l, a)|
                self.post1(l, *a)
                    .any(|l2| s(l2.index()))
            )
    }
}

impl<const N: usize> DGame<N> {
    fn node(&self, l: NodeIndex) -> &DNode<N> {
        self.graph.node_weight(l).unwrap()
    }
}

impl<const N: usize> Default for DGame<N> {
    fn default() -> Self {
        Self {
            graph: Graph::default(),
            l0: node_index(0),
            n_actions: 0,
            obs: DObs::default_array()
        }
    }
}

impl<const N: usize> fmt::Debug for DGame<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ns = self.graph.node_references().format_with(", ", |(i, n), f| {
            if let Some(debug) = &n.debug {
                f(&format_args!("{}:{}:{}", i.index(), debug, if n.is_winning {"W"} else {"-"}))
            } else {
                f(&format_args!("{}:{}", i.index(), if n.is_winning {"W"} else {"-"}))
            }
        });

        let os = self.obs.iter().enumerate().format_with("\n    ", |(i, o), f|
            f(&format_args!("Obs[{}]: {:?}", i, o))
        );
        
        let es = self.graph.edge_references()
            .format_with(", ", |e, f|
                f(&format_args!("({}->{}, {:?})",
                    e.source().index(),
                    e.target().index(),
                    e.weight()
                ))
            );

        write!(f, "DGame {{\n")?;
        write!(f, "    l0: {}, n_agents: {}, n_actions: {}\n", self.l0.index(), N, self.n_actions)?;
        write!(f, "    Nodes: [{}]\n    {}\n    Edges: [{}]\n", ns, os, es)?;
        write!(f, "}}")
    }
}
