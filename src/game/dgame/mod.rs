use fixedbitset::FixedBitSet;
use petgraph::{visit::{GraphBase, Visitable, IntoNeighborsDirected, IntoNeighbors}, graph::{DefaultIx, NodeIndex, IndexType, EdgeIndex, Neighbors, node_index}, Graph, Directed, Direction};

use self::{index::{ObsIndex, ActionIndex}, edge::DEdge, obs::DObs, node::DNode};

use super::{Game, IIGame};

mod index;
mod node;
mod edge;
mod obs;

#[derive(Debug)]
pub struct DGame<Ix: IndexType = DefaultIx> {
    pub graph: Graph<DNode<Ix>, DEdge<Ix>, Directed, Ix>,
    pub l0: NodeIndex<Ix>,
    pub obs: Vec<DObs<Ix>>
}

impl<Ix: IndexType> GraphBase for DGame<Ix> {
    type NodeId = NodeIndex<Ix>;
    type EdgeId = EdgeIndex<Ix>;
}

impl<Ix: IndexType> Game for DGame<Ix> {
    type ActionId = ActionIndex<Ix>;

    fn l0(&self) -> Self::NodeId {
        self.l0
    }
}

impl<Ix: IndexType> IIGame for DGame<Ix> {
    type ObsId = ObsIndex<Ix>;

    fn observe(&self, l: Self::NodeId) -> Self::ObsId {
        self.node(l).obs
    }
}

impl<'a, Ix: IndexType> IntoNeighbors for &'a DGame<Ix> {
    type Neighbors = Neighbors<'a, DEdge<Ix>, Ix>;
    
    fn neighbors(self, l: NodeIndex<Ix>) -> Self::Neighbors {
        self.graph.neighbors(l)
    }
}

impl<'a, Ix: IndexType> IntoNeighborsDirected for &'a DGame<Ix> {
    type NeighborsDirected = Neighbors<'a, DEdge<Ix>, Ix>;
    
    fn neighbors_directed(self, l: NodeIndex<Ix>, dir: Direction) -> Self::NeighborsDirected {
        self.graph.neighbors_directed(l, dir)
    }
}

impl<Ix: IndexType> Visitable for DGame<Ix> {
    type Map = FixedBitSet;

    fn visit_map(&self) -> Self::Map {
        FixedBitSet::with_capacity(self.graph.node_count())
    }

    fn reset_map(&self, map: &mut Self::Map) {
        map.clear();
    }
}

impl<Ix: IndexType> DGame<Ix> {
    pub fn add_node<O>(&mut self, is_winning: bool, o: O) -> NodeIndex<Ix>
    where
        O: Into<ObsIndex<Ix>>
    {
        let o = o.into();
        let n = self.graph.add_node(DNode::new(is_winning, o));
        
        if self.obs.len() <= o.index() {
            while self.obs.len() < o.index() {
                self.obs.push(DObs::default());
            }
            self.obs.push(DObs::new(vec![n]));
        } else {
            self.obs[o.index()].set.push(n);
        }

        n
    }

    pub fn add_edge<I, J, A>(&mut self, i: I, j: J, a: A) -> EdgeIndex<Ix>
    where
        I: Into<NodeIndex<Ix>>,
        J: Into<NodeIndex<Ix>>,
        A: IntoIterator<Item=Ix>
    {
        let itr = a.into_iter().map(|x| x.into());
        self.graph.add_edge(i.into(), j.into(), DEdge::new(Vec::from_iter(itr)))
    }

    fn node(&self, l: NodeIndex<Ix>) -> &DNode<Ix> {
        self.graph.node_weight(l).unwrap()
    }
}

impl<Ix: IndexType> Default for DGame<Ix> {
    fn default() -> Self {
        Self {
            graph: Graph::default(),
            l0: node_index(0),
            obs: Vec::new()
        }
    }
}
