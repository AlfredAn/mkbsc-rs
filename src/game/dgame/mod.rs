use std::{fmt, slice, iter};

use fixedbitset::FixedBitSet;
use petgraph::{visit::{GraphBase, Visitable, IntoNeighborsDirected, IntoNeighbors, IntoNodeReferences, EdgeRef, Data, IntoEdgeReferences, IntoEdges}, graph::{NodeIndex, IndexType, EdgeIndex, Neighbors, node_index, EdgeReference, EdgeReferences}, Graph, Directed, Direction};
use array_init::array_init;
use itertools::Itertools;

use self::{index::{ObsIndex, ActionIndex, AgentIndex}, edge::DEdge, obs::DObs, node::DNode};

use super::{Game, IIGame, MAGame, MAGIIAN, macros::{derive_ma, derive_ii, derive_magiian}};

use crate::game::macros;

pub mod index;
pub mod node;
pub mod edge;
pub mod obs;
pub mod from_game;
pub mod builder;
pub mod generic_builder;

type GraphType<Ix, const N_AGT: usize>
    = Graph<DNode<Ix, N_AGT>, DEdge<Ix, N_AGT>, Directed, Ix>;

pub trait DGameType<const N_AGT: usize>: Game + Default {
    type Ix: IndexType;

    fn graph(&self) -> &GraphType<Self::Ix, N_AGT>;
    fn graph_mut(&mut self) -> &mut GraphType<Self::Ix, N_AGT>;
    fn l0_mut(&mut self) -> &mut Self::NodeId;
    fn obs(&self) -> Option<&[Vec<DObs<Self::Ix>>; N_AGT]>;
    fn obs_mut(&mut self) -> Option<&mut [Vec<DObs<Self::Ix>>; N_AGT]>;
}

macro_rules! impl_dgametype_obs {
    ($na:tt, ImperfectInformation) => {
        fn obs(&self) -> Option<&[Vec<DObs<Self::Ix>>; $na]> { Some(&self.obs) }
        fn obs_mut(&mut self) -> Option<&mut [Vec<DObs<Self::Ix>>; $na]> { Some(&mut self.obs) }
    };
    ($na:tt, PerfectInformation) => {
        fn obs(&self) -> Option<&[Vec<DObs<Self::Ix>>; $na]> { None }
        fn obs_mut(&mut self) -> Option<&mut [Vec<DObs<Self::Ix>>; $na]> { None }
    };
}

macro_rules! impl_game {
    ($name:ident, $type:ty, $it:ident, $ac:ident, $na:tt, ($($tp:tt)*), ($($obs:tt)*), ($($obs_def:tt)*)) => {

#[derive(Clone)]
pub struct $name<$($tp)*> {
    graph: GraphType<Ix, $na>,
    l0: NodeIndex<Ix>,
    $($obs)*
}

impl<$($tp)*> DGameType<$na> for $type {
    type Ix = Ix;

    fn graph(&self) -> &GraphType<Self::Ix, $na> { &self.graph }
    fn graph_mut(&mut self) -> &mut GraphType<Self::Ix, $na> { &mut self.graph }
    fn l0_mut(&mut self) -> &mut Self::NodeId { &mut self.l0 }
    impl_dgametype_obs!($na, $it);
}

impl<$($tp)*> GraphBase for $type {
    type NodeId = NodeIndex<Ix>;
    type EdgeId = EdgeIndex<Ix>;
}

impl<$($tp)*> Data for $type {
    type NodeWeight = DNode<Ix, $na>;
    type EdgeWeight = DEdge<Ix, $na>;
}

impl<$($tp)*> Game for $type {
    type ActionId = [ActionIndex<Ix>; $na];
    type Actions<'a> = iter::Copied<slice::Iter<'a, Self::ActionId>>;
    type Successors<'a> = impl Iterator<Item=Self::EdgeId>;

    fn l0(&self) -> Self::NodeId {
        self.l0
    }

    fn action(&self, e: Self::EdgeId) -> Self::Actions<'_> {
        self.graph[e].act.iter().copied()
    }

    fn is_winning(&self, n: Self::NodeId) -> bool {
        self.node(n).is_winning
    }

    fn successors(&self, n: Self::NodeId) -> Self::Successors<'_> {
        self.graph.edges(n).map(|e| e.id())
    }

    fn source(&self, e: Self::EdgeId) -> Self::NodeId {
        self.graph.edge_endpoints(e).unwrap().0
    }

    fn target(&self, e: Self::EdgeId) -> Self::NodeId {
        self.graph.edge_endpoints(e).unwrap().1
    }
}

impl<'a, $($tp)*> IntoNeighbors for &'a $type {
    type Neighbors = Neighbors<'a, DEdge<Ix, $na>, Ix>;
    
    fn neighbors(self, l: Self::NodeId) -> Self::Neighbors {
        self.graph.neighbors(l)
    }
}

impl<'a, $($tp)*> IntoNeighborsDirected for &'a $type {
    type NeighborsDirected = Neighbors<'a, DEdge<Ix, $na>, Ix>;
    
    fn neighbors_directed(self, l: Self::NodeId, dir: Direction) -> Self::NeighborsDirected {
        self.graph.neighbors_directed(l, dir)
    }
}

impl<'a, $($tp)*> IntoEdgeReferences for &'a $type {
    type EdgeRef = EdgeReference<'a, Self::EdgeWeight, Ix>;
    type EdgeReferences = EdgeReferences<'a, Self::EdgeWeight, Ix>;

    fn edge_references(self) -> Self::EdgeReferences {
        self.graph.edge_references()
    }
}

impl<'a, $($tp)*> IntoEdges for &'a $type {
    type Edges = <&'a GraphType<Ix, $na> as IntoEdges>::Edges;

    fn edges(self, n: Self::NodeId) -> Self::Edges {
        self.graph.edges(n)
    }
}

impl<$($tp)*> Visitable for $type {
    type Map = FixedBitSet;

    fn visit_map(&self) -> Self::Map {
        FixedBitSet::with_capacity(self.graph.node_count())
    }

    fn reset_map(&self, map: &mut Self::Map) {
        map.clear();
    }
}

impl<$($tp)*> $type {
    fn node(&self, l: NodeIndex<Ix>) -> &DNode<Ix, $na> {
        self.graph.node_weight(l).unwrap()
    }
}

impl<$($tp)*> Default for $type {
    fn default() -> Self {
        Self {
            graph: Graph::default(),
            l0: node_index(0),
            $($obs_def)*
        }
    }
}

impl<$($tp)*> fmt::Debug for $type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ns = self.graph.node_references().format_with(", ", |(i, n), f|
            f(&format_args!("{}:{}", i.index(), if n.is_winning {"W"} else {"-"}))
        );

        let os = debug_obs!(self, $it);
        
        let es = self.graph.edge_references()
            .format_with(", ", |e, f|
                f(&format_args!("({}->{}, {:?})",
                    e.source().index(),
                    e.target().index(),
                    e.weight()
                ))
            );

        write!(f, "DGame {{\n")?;
        write!(f, "    l0: {}, n_agents: {}\n", self.l0.index(), self.n_agents())?;
        write!(f, "    Nodes: [{}]\n    {}\n    Edges: [{}]\n", ns, os, es)?;
        write!(f, "}}")
    }
}

};}

macro_rules! debug_obs {
    ($self:ident, PerfectInformation) => {
        ""
    }; ($self:ident, ImperfectInformation) => {
        $self.obs.iter().enumerate().format_with("\n    ", |(i, o), f|
            f(&format_args!("Obs[{}]: {:?}", i, o))
            )
    };
}

macro_rules! impl_iigame {
    ($name:ident, $type:ty, $na:tt, ($($tp:tt)*)) => {
    impl<$($tp)*> IIGame for $type {
        type ObsId = [ObsIndex<Ix>; $na];

        fn observe(&self, l: Self::NodeId) -> Self::ObsId {
            self.node(l).obs
        }
    }
};}

macro_rules! impl_magame {
    ($name:ident, $type:ty, $na:tt, ($($tp:tt)*)) => {
    impl<$($tp)*> MAGame for $type {
        type AgentId = AgentIndex<Ix>;
        type AgentActId = ActionIndex<Ix>;

        fn n_agents(&self) -> usize {
            $na
        }

        fn act_i(&self, act: Self::ActionId, agt: Self::AgentId) -> Self::AgentActId {
            act[agt.index()]
        }
    }
};}

macro_rules! impl_magiian {
    ($name:ident, $type:ty, $na:tt, ($($tp:tt)*)) => {
    impl<$($tp)*> MAGIIAN for $type {
        type AgentObsId = ObsIndex<Ix>;

        fn obs_i(&self, obs: Self::ObsId, agt: Self::AgentId) -> Self::AgentObsId {
            obs[agt.index()]
        }
    }
};}

impl_game!(DMAGIIAN, DMAGIIAN<Ix, N_AGT>, ImperfectInformation, MultiAgent, N_AGT, (Ix: IndexType, const N_AGT: usize), (obs: [Vec<DObs<Ix>>; N_AGT]), (obs: DObs::default_array()));
impl_iigame!(DMAGIIAN, DMAGIIAN<Ix, N_AGT>, N_AGT, (Ix: IndexType, const N_AGT: usize));
impl_magame!(DMAGIIAN, DMAGIIAN<Ix, N_AGT>, N_AGT, (Ix: IndexType, const N_AGT: usize));
impl_magiian!(DMAGIIAN, DMAGIIAN<Ix, N_AGT>, N_AGT, (Ix: IndexType, const N_AGT: usize));

impl_game!(DIIGame, DIIGame<Ix>, ImperfectInformation, SingleAgent, 1, (Ix: IndexType), (obs: [Vec<DObs<Ix>>; 1]), (obs: DObs::default_array()));
impl_iigame!(DIIGame, DIIGame<Ix>, 1, (Ix: IndexType));
derive_ma!(DIIGame<Ix>, Ix: IndexType);
derive_magiian!(DIIGame<Ix>, Ix: IndexType);

impl_game!(DMAGame, DMAGame<Ix, N_AGT>, PerfectInformation, MultiAgent, N_AGT, (Ix: IndexType, const N_AGT: usize), (), ());
impl_magame!(DMAGame, DMAGame<Ix, N_AGT>, N_AGT, (Ix: IndexType, const N_AGT: usize));
derive_ii!(DMAGame<Ix, N_AGT>, Ix: IndexType, const N_AGT: usize);
derive_magiian!(DMAGame<Ix, N_AGT>, Ix: IndexType, const N_AGT: usize);

impl_game!(DGame, DGame<Ix>, PerfectInformation, SingleAgent, 1, (Ix: IndexType), (), ());
derive_ii!(DGame<Ix>, Ix: IndexType);
derive_ma!(DGame<Ix>, Ix: IndexType);
derive_magiian!(DGame<Ix>, Ix: IndexType);
