use fixedbitset::FixedBitSet;
use petgraph::Incoming;
use crate::game::*;
use std::fmt;

use petgraph::{visit::*, graph::node_index, Graph, Directed};
use array_init::array_init;
use itertools::Itertools;

use self::{edge::DEdge, obs::DObs, node::DNode};

use super::Game;

use crate::util::*;

pub mod index;
pub mod node;
pub mod edge;
pub mod obs;
pub mod from_game;
pub mod builder;
pub mod generic_builder;

pub use index::*;
pub use node::*;
pub use edge::*;
pub use obs::*;
pub use generic_builder::*;

type GraphType<const N: usize>
    = Graph<DNode<N>, DEdge<N>, Directed>;

#[derive(Clone)]
pub struct DGame<const N: usize> {
    pub graph: GraphType<N>,
    pub l0: NodeIndex,
    pub n_actions: usize,
    pub obs: [Vec<DObs>; N]
}

impl<const N: usize> Game<N> for DGame<N> {
    type Loc = NodeIndex;
    type Act = ActionIndex;

    fn l0(&self) -> &Self::Loc {
        &self.l0
    }

    fn actions(&self) -> Itr<[Self::Act; N]> {
        Box::new(range_power::<N>(0..self.n_actions).map(|x| array_init(|i| action_index(x[i]))))
    }

    fn is_winning(&self, n: &Self::Loc) -> bool {
        self.node(*n).is_winning
    }

    fn post(&self, n: &Self::Loc, a: [Self::Act; N]) -> Itr<Self::Loc> {
        Box::new(self.graph.edges(*n).filter(move |e| e.weight().act == a).map(|e| e.target()))
    }

    type Obs = ObsIndex;

    fn observe(&self, l: &Self::Loc) -> [Self::Obs; N] {
        self.node(*l).obs
    }

    type Agt = AgtIndex;

    fn actions_i(&self, _: Self::Agt) -> Itr<Self::Act> {
        Box::new((0..self.n_actions).map(|a| action_index(a)))
    }

    fn debug_string(&self, _: &Self::Loc) -> Option<String> {
        None//self.node(*l).debug.clone()
    }

    fn dgame(&self) -> Cow<Self> {
        Cow::Borrowed(self)
    }

    fn into_dgame(self) -> Self {
        self
    }
}

impl<'a> Game1 for DGame<1> {
    fn all_strategies1(&self) -> AllStrategies1 {
        let w = find_memoryless_strategies(self);
        AllStrategies1::new(&w, self.n_actions)
    }
}

impl<const N: usize> HasVisitSet<N> for DGame<N> {
    type VisitSet = FixedBitSet;
    fn visit_set(&self) -> FixedBitSet {
        FixedBitSet::with_capacity(self.graph.node_count())
    }
}

impl VisitSet<NodeIndex> for FixedBitSet {
    fn insert(&mut self, l: NodeIndex) -> bool {
        !self.put(l.borrow().index())
    }
    
    fn clear(&mut self) {
        FixedBitSet::clear(self);
    }

    fn contains(&self, l: &NodeIndex) -> bool {
        self[l.borrow().index()]
    }
}

impl DGame<1> {
    pub fn pre<'b>(&'b self, s: impl IntoIterator<Item=NodeIndex> + 'b, a: ActionIndex) -> impl Iterator<Item=NodeIndex> + 'b {
        s.into_iter()
            .map(move |n| self.graph.edges_directed(n, Incoming)
                .filter(move |e| e.weight().act == [a])
                .map(|e| e.source())
            ).flatten()
    }

    pub fn obs_set1(&self, obs: ObsIndex) -> &[NodeIndex] {
        &*self.obs[0][obs.index()].set
    }
}

impl<const N: usize> DGame<N> {
    fn node(&self, l: NodeIndex) -> &DNode<N> {
        self.graph.node_weight(l).unwrap()
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
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
            /*if let Some(debug) = &n.debug {
                f(&format_args!("{}:{}:{}", i.index(), debug, if n.is_winning {"W"} else {"-"}))
            } else {*/
                f(&format_args!("{}:{}", i.index(), if n.is_winning {"W"} else {"-"}))
            //}
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
