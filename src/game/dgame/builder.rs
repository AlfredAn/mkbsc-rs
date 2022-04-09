/*use std::cmp::max;

use petgraph::graph::IndexType;

use super::{obs::DObs, GraphType, index::{NodeIndex, ObsIndex, EdgeIndex, obs_index, action_index}, DGame, node::DNode, edge::DEdge};


#[derive(Debug, Clone)]
pub struct Builder<const N_AGT: usize> {
    graph: GraphType<N_AGT>,
    l0: NodeIndex,
    n_actions: usize,
    obs: [Vec<DObs>; N_AGT]
}

impl<const N_AGT: usize> Default for Builder<N_AGT> {
    fn default() -> Self {
        Self {
            graph: Default::default(),
            l0: Default::default(),
            n_actions: Default::default(),
            obs: DObs::default_array()
        }
    }
}

impl<const N_AGT: usize> Builder<N_AGT> {
    pub fn add_node<O>(&mut self, is_winning: bool, o: [O; N_AGT]) -> NodeIndex
    where
        O: Into<ObsIndex>
    {
        let o = o.map(|x| x.into());
        let n = self.graph.add_node(DNode::new(is_winning, o, None));

        println!("{:?}", (n, o));
        
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

    pub fn add_node_pi(&mut self, is_winning: bool) -> NodeIndex {
        self.add_node(is_winning, [obs_index(self.graph.node_count()); N_AGT])
    }

    pub fn add_edge<I, J, A>(&mut self, i: I, j: J, a: A) -> EdgeIndex
    where
        I: Into<NodeIndex>,
        J: Into<NodeIndex>,
        A: IntoIterator<Item=[u32; N_AGT]>
    {
        let (v, mx) = a.into_iter()
            .fold((Vec::new(), 0), |(mut v, mx), a| {
                v.push(a.map(|aa| action_index(aa.index())));
                let &a_max = a.iter().max().unwrap();
                (v, max(mx, a_max))
            });

        self.n_actions = max(self.n_actions, mx.index()+1);
        self.graph.add_edge(i.into(), j.into(), DEdge::new(v))
    }

    pub fn l0<I>(&mut self, l0: I) -> &mut Self
    where
        I: Into<NodeIndex>
    {
        self.l0 = l0.into();
        self
    }

    pub fn build(self) -> DGame<N_AGT> {
        DGame {
            graph: self.graph,
            l0: self.l0,
            n_actions: self.n_actions,
            obs: self.obs
        }
    }
}

impl Builder<1> {
    pub fn add_node1<O>(&mut self, is_winning: bool, o: O) -> NodeIndex
    where
        O: Into<ObsIndex>
    {
        self.add_node(is_winning, [o])
    }

    pub fn add_edge1<I, J, A>(&mut self, i: I, j: J, a: A) -> EdgeIndex
    where
        I: Into<NodeIndex>,
        J: Into<NodeIndex>,
        A: IntoIterator<Item=u32>
    {
        self.add_edge(i, j, a.into_iter().map(|x| [x]))
    }
}
*/