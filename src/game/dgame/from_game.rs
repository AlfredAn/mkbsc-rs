use std::{collections::HashMap, hash::Hash, fmt::Debug, error::Error, iter};

use array_init::from_iter;
use petgraph::{graph::IndexType, visit::{Visitable, IntoEdges, VisitMap, EdgeRef, GraphBase}};

use crate::game::Game;

use super::{DGame, builder::Builder, generic_builder::GenericBuilder, index::{agent_index, NodeIndex}, node::DNode};

impl<Ix, const N: usize> DGame<Ix, N>
where
    Ix: IndexType
{
    pub fn from_game<'a, G: Game<'a, N>>(g: G, stop_on_win: bool) -> anyhow::Result<DGame<Ix, N>> {
        let mut b = GenericBuilder::default();
        b.l0(g.l0().clone())?;

        //println!("start");

        let mut stack = vec![g.l0().clone()];
        while let Some(l) = stack.pop() {
            if b.has_node(&l) {
                //println!("skipping");
                continue;
            }

            //println!("\n{:?}", l);

            let is_winning = g.is_winning(&l);
            
            b.node_dbg(l.clone(), is_winning, g.debug_string(&l).as_deref())?;
            //println!("{:?}", g.debug_string(&l));
            let obs = g.observe(&l);
            for (i, o) in obs.into_iter().enumerate() {
                b.obs(o, i, iter::once(l.clone()));
            }

            for a in g.actions() {
                //println!("{:?}", a);
                for n in g.post(&l, a) {
                    //println!("post: {:?}", n);
                    if !(stop_on_win && is_winning) {
                        b.edge(l.clone(), n.clone(), iter::once((0..N).map(|i| a[i])))?;
                    }
                    stack.push(n);
                }
            }
        }

        b.build()
    }
}
