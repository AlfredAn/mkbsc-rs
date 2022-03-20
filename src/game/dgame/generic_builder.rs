use std::{marker::PhantomData, collections::HashMap, hash::Hash, fmt::Debug};

use anyhow::bail;
use array_init::from_iter;
use arrayvec::ArrayVec;
use petgraph::{graphmap::GraphMap, Graph, Directed, graph::{DefaultIx, IndexType, node_index}, visit::EdgeRef};
use itertools::Itertools;

use crate::game::dgame::{node::DNode, obs::DObs};

use super::{index::{EdgeIndex, NodeIndex, ActionIndex, action_index}, DGame, edge::DEdge};

type NI = NodeIndex<usize>;
type EI = EdgeIndex<usize>;
type AI = ActionIndex<usize>;

pub struct GenericBuilder<N, A, O, const N_AGT: usize> {
    graph: Graph<Option<bool>, Vec<[AI; N_AGT]>, Directed, usize>,
    l0: Option<NI>,
    nodes: HashMap<N, NI>,
    edges: HashMap<(N, N), EI>,
    actions: HashMap<A, AI>,
    n_actions: usize,

    _o: PhantomData<O>
}

impl<Loc, Act, Obs, const N: usize> Default for GenericBuilder<Loc, Act, Obs, N> {
    fn default() -> Self {
        Self {
            graph: Default::default(),
            l0: Default::default(),
            nodes: Default::default(),
            edges: Default::default(),
            _o: Default::default(),
            actions: Default::default(),
            n_actions: 0,
        }
    }
}

impl<Loc, Act, Obs, const N: usize> GenericBuilder<Loc, Act, Obs, N>
where
    Loc: Clone + Eq + Hash,
    Act: Copy + Eq + Hash
{
    fn _add_node(&mut self, node: Loc, is_winning: Option<bool>) -> anyhow::Result<NI> {
        if let Some(&n) = self.nodes.get(&node) {
            if self.graph[n] == None {
                self.graph[n] = is_winning;
                return Ok(n);
            } else if is_winning == None {
                return Ok(n);
            } else {
                bail!("Trying to add node twice.");
            }
        }
        let n = self.graph.add_node(is_winning);
        self.nodes.insert(node, n);
        Ok(n)
    }

    pub fn add_node(&mut self, node: Loc, is_winning: bool) -> anyhow::Result<NI> {
        self._add_node(node, Some(is_winning))
    }

    pub fn node(&self, node: &Loc) -> Option<NI> {
        self.nodes.get(&node).map(|&n| n)
    }

    pub fn has_node(&self, node: &Loc) -> bool {
        if let Some(&n) = self.nodes.get(&node) {
            self.graph[n].is_some()
        } else {
            false
        }
    }

    fn action(&mut self, act: Act) -> AI {
        if let Some(&a) = self.actions.get(&act) {
            a
        } else {
            let a = action_index(self.actions.len());
            self.actions.insert(act, a);
            a
        }
    }

    fn actions<As>(&mut self, acts: As) -> anyhow::Result<[AI; N]>
    where
        As: IntoIterator<Item=Act>
    {
        let itr = acts.into_iter().map(|a| self.action(a));
        let result = from_iter(itr);
        if let Some(result) = result {
            return Ok(result);
        }
        bail!("Action contains wrong number of elements");
    }

    pub fn add_edge<As1, As2>(&mut self, from: Loc, to: Loc, act: As2) -> anyhow::Result<EI>
    where
        As1: IntoIterator<Item=Act>,
        As2: IntoIterator<Item=As1>
    {
        let f = self._add_node(from.clone(), None)?;
        let t = self._add_node(to.clone(), None)?;

        if let Some(&e) = self.edges.get(&(from.clone(), to.clone())) {
            Ok(e)
        } else {
            let mut a_vec = Vec::new();
            for a in act.into_iter() {
                let a = self.actions(a)?;
                a_vec.push(a);
            }
                
            let e = self.graph.add_edge(f, t, a_vec);
            self.edges.insert((from, to), e);
            Ok(e)
        }
    }

    pub fn build<Ix>(&self) -> anyhow::Result<DGame<Ix, N>>
    where
        Ix: IndexType
    {
        let mut g = DGame::default();

        for n in self.graph.node_indices() {
            let is_winning = if let Some(w) = self.graph[n] {
                w
            } else {
                bail!("Edge references node that does not exist");
            };
            let n2 = g.graph.add_node(DNode::new(
                is_winning,
                [Default::default(); N]
            ));
            assert_eq!(n.index(), n2.index());
        }

        for e in self.graph.edge_references() {
            let e2 = g.graph.add_edge(
                node_index(e.source().index()),
                node_index(e.target().index()),
                DEdge::new(
                    e.weight().iter().map(|a|
                        a.map(|aa|
                            action_index(aa.index())
                        )
                    ).collect()
                )
            );
            assert_eq!(e.id().index(), e2.index());
        }
        
        Ok(g)
    }
}
