use std::{fmt, slice, iter};

use fixedbitset::FixedBitSet;
use petgraph::{visit::*, graph::{IndexType, Neighbors, node_index, EdgeReference, EdgeReferences}, Graph, Directed, Direction};
use array_init::array_init;
use itertools::Itertools;

use self::{index::*, edge::DEdge, obs::DObs, node::DNode};

use super::{Game, IIGame, MAGame, MAGIIAN, macros::{derive_ma, derive_ii, derive_magiian}};

use crate::{game::macros, util::*};

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
    fn l0_mut(&mut self) -> &mut Self::Loc;
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
    n_actions: usize,
    $($obs)*
}

impl<$($tp)*> DGameType<$na> for $type {
    type Ix = Ix;

    fn graph(&self) -> &GraphType<Self::Ix, $na> { &self.graph }
    fn graph_mut(&mut self) -> &mut GraphType<Self::Ix, $na> { &mut self.graph }
    fn l0_mut(&mut self) -> &mut Self::Loc { &mut self.l0 }
    impl_dgametype_obs!($na, $it);
}

impl<$($tp)*> Game for $type {
    type Loc = NodeIndex<Ix>;
    type Act = [ActionIndex<Ix>; $na];
    type Actions<'a> = impl Iterator<Item=Self::Act>;
    type Post<'a> = impl Iterator<Item=Self::Loc>;

    fn l0(&self) -> Self::Loc {
        self.l0
    }

    fn actions(&self) -> Self::Actions<'_> {
        map_array(range_power(0..self.n_actions), |&a| action_index(a))
    }

    fn is_winning(&self, n: Self::Loc) -> bool {
        self.node(n).is_winning
    }

    fn post(&self, n: Self::Loc, a: Self::Act) -> Self::Post<'_> {
        self.graph.edges(n).filter(move |e| e.weight().act.contains(&a)).map(|e| e.target())
    }

    type Obs = [ObsIndex<Ix>; $na];

    fn observe(&self, l: Self::Loc) -> Self::Obs {
        self.node(l).obs
    }

    type Agent = AgentIndex<Ix>;
    type AgentAct = ActionIndex<Ix>;
    type ActionsI<'a> = impl Iterator<Item=Self::AgentAct>;

    fn n_agents(&self) -> usize {
        $na
    }

    fn act_i(&self, act: Self::Act, agt: Self::Agent) -> Self::AgentAct {
        act[agt.index()]
    }

    fn actions_i(&self, agt: Self::Agent) -> Self::ActionsI<'_> {
        (0..self.n_actions).map(|a| action_index(a))
    }

    type AgentObs = ObsIndex<Ix>;

    fn obs_i(&self, obs: Self::Obs, agt: Self::Agent) -> Self::AgentObs {
        obs[agt.index()]
    }
}

impl<$($tp)*> IIGame for $type {}
impl<$($tp)*> MAGame for $type {}
impl<$($tp)*> MAGIIAN for $type {}

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

impl_game!(DGame, DGame<Ix, N_AGT>, ImperfectInformation, MultiAgent, N_AGT, (Ix: IndexType, const N_AGT: usize), (obs: [Vec<DObs<Ix>>; N_AGT]), (obs: DObs::default_array()));
