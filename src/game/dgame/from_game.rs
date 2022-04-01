use std::fmt;
use std::fmt::Display;
use std::{collections::HashMap, hash::Hash, fmt::Debug, error::Error, iter};

use array_init::from_iter;
use petgraph::{graph::IndexType, visit::*};

use crate::game::Game;

use super::{DGame, builder::Builder, generic_builder::GenericBuilder, index::{agent_index, NodeIndex}, node::DNode};

impl<Ix, const N: usize> DGame<Ix, N>
where
    Ix: IndexType
{
    pub fn from_game<'a, G>(g: G, stop_on_win: bool) -> anyhow::Result<DGame<Ix, N>>
    where
        G: Game<'a, N>
    {
        Self::from_game_labels(g, stop_on_win, |g, l| g.debug_string(l))
    }

    pub fn from_game_labels<'a, G, F>(g: G, stop_on_win: bool, mut f: F) -> anyhow::Result<DGame<Ix, N>>
    where
        G: Game<'a, N>,
        F: FnMut(&G, &G::Loc) -> Option<String>
    {
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
            
            b.node_dbg(l.clone(), is_winning, f(&g, &l))?;
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

pub fn from_game<'a, G, const N: usize>(g: G) -> DGame<u32, N>
where
    G: Game<'a, N>
{
    DGame::from_game(g, false).unwrap()
}
