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

pub trait DGameType<'a, const N: usize>: Game<'a, N> + Default {
    type Ix: IndexType;

    fn graph(&self) -> &GraphType<Self::Ix, N>;
    fn graph_mut(&mut self) -> &mut GraphType<Self::Ix, N>;
    fn l0_mut(&mut self) -> &mut Self::Loc;
    fn obs(&self) -> Option<&[Vec<DObs<Self::Ix>>; N]>;
    fn obs_mut(&mut self) -> Option<&mut [Vec<DObs<Self::Ix>>; N]>;
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
    n_actions: usize,
    $($obs)*
}

impl<'a, $($tp)*> DGameType<'a, $na> for $type {
    type Ix = Ix;

    fn graph(&self) -> &GraphType<Self::Ix, $na> { &self.graph }
    fn graph_mut(&mut self) -> &mut GraphType<Self::Ix, $na> { &mut self.graph }
    fn l0_mut(&mut self) -> &mut Self::Loc { &mut self.l0 }
    impl_dgametype_obs!($na, $it);
}

impl<'a, $($tp)*> Game<'a, $na> for $type {
    type Loc = NodeIndex<Ix>;
    type Act = ActionIndex<Ix>;
    type Actions = impl Iterator<Item=[Self::Act; $na]>;
    type Post = impl Iterator<Item=Self::Loc>;

    fn l0(&self) -> Self::Loc {
        self.l0
    }

    fn actions(&self) -> Self::Actions {
        map_array(range_power(0..self.n_actions), |&a| action_index(a))
    }

    fn is_winning(&self, n: Self::Loc) -> bool {
        self.node(n).is_winning
    }

    fn post(&'a self, n: Self::Loc, a: [Self::Act; $na]) -> Self::Post {
        self.graph.edges(n).filter(move |e| e.weight().act.contains(&a)).map(|e| e.target())
    }

    type Obs = [ObsIndex<Ix>; $na];

    fn observe(&self, l: Self::Loc) -> Self::Obs {
        self.node(l).obs
    }

    type Agent = AgentIndex<Ix>;
    type ActionsI = impl Iterator<Item=Self::Act>;

    fn actions_i(&self, agt: Self::Agent) -> Self::ActionsI {
        (0..self.n_actions).map(|a| action_index(a))
    }

    type AgentObs = ObsIndex<Ix>;

    fn obs_i(&self, obs: Self::Obs, agt: Self::Agent) -> Self::AgentObs {
        obs[agt.index()]
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
            n_actions: 0,
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
        write!(f, "    l0: {}, n_agents: {}\n", self.l0.index(), $na)?;
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

impl_game!(DGame, DGame<Ix, N>, ImperfectInformation, MultiAgent, N, (Ix: IndexType, const N: usize), (obs: [Vec<DObs<Ix>>; N]), (obs: DObs::default_array()));
