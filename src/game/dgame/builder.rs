use petgraph::graph::IndexType;

use super::{obs::DObs, GraphType, index::{NodeIndex, ObsIndex, EdgeIndex, obs_index}, DGame, node::DNode, edge::DEdge};


#[derive(Debug, Clone)]
pub struct Builder<Ix: IndexType, const N_AGT: usize> {
    graph: GraphType<Ix, N_AGT>,
    l0: NodeIndex<Ix>,
    n_actions: usize,
    obs: [Vec<DObs<Ix>>; N_AGT]
}

impl<Ix: IndexType, const N_AGT: usize> Default for Builder<Ix, N_AGT> {
    fn default() -> Self {
        Self {
            graph: Default::default(),
            l0: Default::default(),
            n_actions: Default::default(),
            obs: DObs::default_array()
        }
    }
}

impl<Ix: IndexType, const N_AGT: usize> Builder<Ix, N_AGT> {
    fn add_node<O>(&mut self, is_winning: bool, o: [O; N_AGT]) -> NodeIndex<Ix>
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

    pub fn add_node_pi(&mut self, is_winning: bool) -> NodeIndex<Ix> {
        self.add_node(is_winning, [obs_index(self.graph.node_count()); N_AGT])
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

    pub fn l0<I>(&mut self, l0: I) -> &mut Self
    where
        I: Into<NodeIndex<Ix>>
    {
        self.l0 = l0.into();
        self
    }

    pub fn build(self) -> DGame<Ix, N_AGT> {
        todo!();
        DGame {
            graph: self.graph,
            l0: self.l0,
            n_actions: self.n_actions,
            obs: self.obs
        }
    }
}

impl<Ix: IndexType> Builder<Ix, 1> {
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
