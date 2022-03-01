use fixedbitset::FixedBitSet;
use petgraph::{visit::{GraphBase, Visitable, IntoNeighborsDirected, IntoNeighbors}, graph::{NodeIndex, IndexType, EdgeIndex, Neighbors, node_index}, Graph, Directed, Direction};
use array_init::array_init;

use self::{index::{ObsIndex, ActionIndex, AgentIndex}, edge::DEdge, obs::DObs, node::DNode};

use super::{Game, IIGame, MAGame, MAGIIAN};

mod index;
mod node;
mod edge;
mod obs;

#[derive(Debug)]
pub struct DGame<Ix: IndexType, const N_AGT: usize> {
    pub graph: Graph<DNode<Ix, N_AGT>, DEdge<Ix, N_AGT>, Directed, Ix>,
    pub l0: NodeIndex<Ix>,
    pub obs: [Vec<DObs<Ix>>; N_AGT],
}

impl<Ix: IndexType, const N_AGT: usize> GraphBase for DGame<Ix, N_AGT> {
    type NodeId = NodeIndex<Ix>;
    type EdgeId = EdgeIndex<Ix>;
}

impl<Ix: IndexType, const N_AGT: usize> Game for DGame<Ix, N_AGT> {
    type ActionId = [ActionIndex<Ix>; N_AGT];

    fn l0(&self) -> Self::NodeId {
        self.l0
    }
}

impl<Ix: IndexType, const N_AGT: usize> IIGame for DGame<Ix, N_AGT> {
    type ObsId = [ObsIndex<Ix>; N_AGT];

    fn observe(&self, l: Self::NodeId) -> Self::ObsId {
        self.node(l).obs
    }
}

impl<Ix: IndexType, const N_AGT: usize> MAGame for DGame<Ix, N_AGT> {
    type AgentId = AgentIndex<Ix>;
    type AgentActId = ActionIndex<Ix>;

    fn n_agents(&self) -> usize {
        N_AGT
    }

    fn act_i(&self, act: Self::ActionId, agt: Self::AgentId) -> Self::AgentActId {
        act[agt.index()]
    }
}

impl<Ix: IndexType, const N_AGT: usize> MAGIIAN for DGame<Ix, N_AGT> {
    type AgentObsId = ObsIndex<Ix>;

    fn obs_i(&self, obs: Self::ObsId, agt: Self::AgentId) -> Self::AgentObsId {
        obs[agt.index()]
    }
}

impl<'a, Ix: IndexType, const N_AGT: usize> IntoNeighbors for &'a DGame<Ix, N_AGT> {
    type Neighbors = Neighbors<'a, DEdge<Ix, N_AGT>, Ix>;
    
    fn neighbors(self, l: Self::NodeId) -> Self::Neighbors {
        self.graph.neighbors(l)
    }
}

impl<'a, Ix: IndexType, const N_AGT: usize> IntoNeighborsDirected for &'a DGame<Ix, N_AGT> {
    type NeighborsDirected = Neighbors<'a, DEdge<Ix, N_AGT>, Ix>;
    
    fn neighbors_directed(self, l: Self::NodeId, dir: Direction) -> Self::NeighborsDirected {
        self.graph.neighbors_directed(l, dir)
    }
}

impl<Ix: IndexType, const N_AGT: usize> Visitable for DGame<Ix, N_AGT> {
    type Map = FixedBitSet;

    fn visit_map(&self) -> Self::Map {
        FixedBitSet::with_capacity(self.graph.node_count())
    }

    fn reset_map(&self, map: &mut Self::Map) {
        map.clear();
    }
}

impl<Ix: IndexType, const N_AGT: usize> DGame<Ix, N_AGT> {
    pub fn add_node<O>(&mut self, is_winning: bool, o: [O; N_AGT]) -> NodeIndex<Ix>
    where
        O: Into<ObsIndex<Ix>>
    {
        let o = o.map(|x| x.into());
        let n = self.graph.add_node(DNode::new(is_winning, o));
        
        for (i, o) in o.iter().enumerate() {
            if self.obs[i].len() <= o.index() {
                while self.obs[i].len() < o.index() {
                    self.obs[i].push(DObs::default());
                }
                self.obs[i].push(DObs::new(vec![n]));
            } else {
                self.obs[i][o.index()].set.push(n);
            }
        }

        n
    }

    pub fn add_edge<I, J, A>(&mut self, i: I, j: J, a: A) -> EdgeIndex<Ix>
    where
        I: Into<NodeIndex<Ix>>,
        J: Into<NodeIndex<Ix>>,
        A: IntoIterator<Item=[Ix; N_AGT]>
    {
        let itr = a.into_iter().map(|x| x.map(|y| y.into()));
        self.graph.add_edge(i.into(), j.into(), DEdge::new(Vec::from_iter(itr)))
    }

    fn node(&self, l: NodeIndex<Ix>) -> &DNode<Ix, N_AGT> {
        self.graph.node_weight(l).unwrap()
    }
}

impl<Ix: IndexType> DGame<Ix, 1> {
    pub fn add_node1<O>(&mut self, is_winning: bool, o: O) -> NodeIndex<Ix>
    where
        O: Into<ObsIndex<Ix>>
    {
        self.add_node(is_winning, [o])
    }

    pub fn add_edge1<I, J, A>(&mut self, i: I, j: J, a: A) -> EdgeIndex<Ix>
    where
        I: Into<NodeIndex<Ix>>,
        J: Into<NodeIndex<Ix>>,
        A: IntoIterator<Item=Ix>
    {
        self.add_edge(i, j, a.into_iter().map(|x| [x]))
    }
}

impl<Ix: IndexType, const N_AGT: usize> Default for DGame<Ix, N_AGT> {
    fn default() -> Self {
        Self {
            graph: Graph::default(),
            l0: node_index(0),
            obs: array_init(|_| Vec::new())
        }
    }
}
