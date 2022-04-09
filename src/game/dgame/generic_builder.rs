use petgraph::visit::EdgeRef;
use std::{collections::HashMap, hash::Hash, cmp::max};

use anyhow::bail;
use array_init::*;
use bimap::BiHashMap;
use petgraph::*;

use crate::game::dgame::{node::DNode, obs::DObs};

use super::{index::*, DGame, edge::DEdge};

type NI = NodeIndex;
type AI = ActionIndex;
type OI = ObsIndex;

pub struct GenericBuilder<Loc, Act, Obs, const N: usize> {
    graph: Graph<(Option<bool>, Option<String>), [AI; N], Directed, u32>,
    l0: Option<NI>,
    nodes: BiHashMap<Loc, NI>,
    actions: HashMap<Act, AI>,
    obs: HashMap<Obs, (OI, [Vec<NI>; N])>,
    labels: Option<Box<dyn Fn(&Loc) -> String>>
}

impl<Loc, Act, Obs, const N: usize> Default for GenericBuilder<Loc, Act, Obs, N>
where
    Loc: Eq + Hash
{
    fn default() -> Self {
        Self {
            graph: Default::default(),
            l0: Default::default(),
            nodes: Default::default(),
            actions: Default::default(),
            obs: Default::default(),
            labels: None
        }
    }
}

impl<Loc, Act, Obs, const N: usize> GenericBuilder<Loc, Act, Obs, N>
where
    Loc: Clone + Eq + Hash,
    Act: Copy + Eq + Hash,
    Obs: Clone + Eq + Hash
{
    fn _node(&mut self, node: Loc, is_winning: Option<bool>, debug: Option<String>) -> anyhow::Result<NI> {
        if let Some(&n) = self.nodes.get_by_left(&node) {
            if self.graph[n].0.is_none() {
                self.graph[n] = (is_winning, debug);
                return Ok(n);
            } else if is_winning.is_none() {
                return Ok(n);
            } else {
                bail!("Trying to add node twice.");
            }
        }
        let n = self.graph.add_node((is_winning, debug.map(|s| s.into())));
        self.nodes.insert(node, n);
        Ok(n)
    }

    pub fn node(&mut self, node: Loc, is_winning: bool) -> anyhow::Result<NI> {
        self._node(node, Some(is_winning), None)
    }

    pub fn node_dbg(&mut self, node: Loc, is_winning: bool, dbg: Option<String>) -> anyhow::Result<NI> {
        self._node(node, Some(is_winning), dbg)
    }

    pub fn get_node(&self, node: &Loc) -> Option<NI> {
        self.nodes.get_by_left(node).map(|&n| n)
    }

    pub fn has_node(&self, node: &Loc) -> bool {
        if let Some(&n) = self.nodes.get_by_left(node) {
            self.graph[n].0.is_some()
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

    pub fn edge<As1, As2>(&mut self, from: Loc, to: Loc, act: As2) -> anyhow::Result<()>
    where
        As1: IntoIterator<Item=Act>,
        As2: IntoIterator<Item=As1>
    {
        let f = self._node(from.clone(), None, None)?;
        let t = self._node(to.clone(), None, None)?;

        for a in act {
            let a = self.actions(a)?;
            self.graph.add_edge(f, t, a);
        }

        Ok(())
    }

    pub fn obs<Ns>(&mut self, obs: Obs, agt: usize, ns: Ns) -> OI
    where
        Ns: IntoIterator<Item=Loc>
    {
        let o;
        if !self.obs.contains_key(&obs) {
            o = OI::new(self.obs.len());
            let val = (o, array_init(|_| Vec::new()));
            self.obs.insert(obs.clone(), val);
        } else {
            o = self.obs.get_mut(&obs).unwrap().0;
        }

        for n in ns {
            let n = self._node(n, None, None).unwrap();
            let (_, v) = self.obs.get_mut(&obs).unwrap();
            v[agt].push(n);
        }

        o
    }

    pub fn l0(&mut self, l0: Loc) -> anyhow::Result<()> {
        let n = self._node(l0, None, None)?;
        self.l0 = Some(n);
        Ok(())
    }

    pub fn labels(&mut self, labels: Box<dyn Fn(&Loc) -> String>) {
        self.labels = Some(labels)
    }

    pub fn build(&self) -> anyhow::Result<DGame<N>> {
        if self.l0.is_none() {
            bail!("l0 is not set");
        }
        let l0 = self.l0.unwrap();
        let mut l0_valid = false;

        let mut g = DGame::<N>::default();

        for n in self.graph.node_indices() {
            let is_winning = if let Some(w) = self.graph[n].0 {
                w
            } else {
                bail!("Edge references node that does not exist");
            };
            let n2 = g.graph.add_node(DNode::new(
                is_winning,
                [Default::default(); N],
                /*{
                    if let Some(lb) = &self.labels {
                        Some(lb(self.nodes.get_by_right(&n).unwrap()))
                    } else if let Some(debug) = &self.graph[n].1 {
                        Some(debug.clone())
                    } else {
                        None
                    }
                }*/
            ));
            if l0 == n {
                g.l0 = n2;
                l0_valid = true;
            }
            assert_eq!(n.index(), n2.index());
        }

        assert!(l0_valid);

        for e in self.graph.edge_references() {
            g.n_actions = max(g.n_actions,
                e.weight().iter()
                    .max().unwrap().index() + 1);
            let e2 = g.graph.add_edge(
                node_index(e.source().index()),
                node_index(e.target().index()),
                DEdge::new(
                    e.weight().map(|a| action_index(a.index()))
                )
            );
            assert_eq!(e.id().index(), e2.index());
        }

        for (o, vs) in self.obs.values() {
            let o = obs_index(o.index());
            for (i, v) in vs.iter().enumerate() {
                let mut v2 = Vec::new();
                for n in v {
                    let n = node_index(n.index());
                    g.graph[n].obs[i] = o;
                    v2.push(n);
                }
                if !v2.is_empty() {
                    g.obs[i].push(DObs::new(v2));
                }
            }
        }
        
        Ok(g)
    }
}
