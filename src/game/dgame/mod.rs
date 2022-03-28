use std::{fmt, slice, iter};

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

type GraphType<Ix, const N: usize>
    = Graph<DNode<Ix, N>, DEdge<Ix, N>, Directed, Ix>;

#[derive(Clone)]
pub struct DGame<Ix: IndexType, const N: usize> {
    pub graph: GraphType<Ix, N>,
    pub l0: NodeIndex<Ix>,
    pub n_actions: usize,
    pub obs: [Vec<DObs<Ix>>; N]
}

impl<'a, Ix: IndexType, const N: usize> Game<'a, N> for DGame<Ix, N> {
    type Loc = NodeIndex<Ix>;
    type Act = ActionIndex<Ix>;

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

    type Obs = ObsIndex<Ix>;

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; N] {
        self.node(*l).obs
    }

    type Agent = AgentIndex<Ix>;

    fn actions_i<'b>(&'b self, _: Self::Agent) -> Itr<'b, Self::Act> where 'a: 'b {
        Box::new((0..self.n_actions).map(|a| action_index(a)))
    }

    fn debug_string(&self, l: &Self::Loc) -> Option<String> {
        self.node(*l).debug.clone()
    }
}

impl<Ix: IndexType, const N: usize> DGame<Ix, N> {
    fn node(&self, l: NodeIndex<Ix>) -> &DNode<Ix, N> {
        self.graph.node_weight(l).unwrap()
    }
}

impl<Ix: IndexType, const N: usize> Default for DGame<Ix, N> {
    fn default() -> Self {
        Self {
            graph: Graph::default(),
            l0: node_index(0),
            n_actions: 0,
            obs: DObs::default_array()
        }
    }
}

impl<Ix: IndexType, const N: usize> fmt::Debug for DGame<Ix, N> {
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
