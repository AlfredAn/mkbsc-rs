#![feature(type_alias_impl_trait)]

#![allow(unused_imports)]
use game::{Game, dgame::DGame};
use games::grid_pursuit::GridPursuitGame;
use itertools::Itertools;
use petgraph::visit::{Walker, Dfs, IntoNeighbors, IntoEdges, EdgeRef, Visitable, VisitMap};

use crate::{games::three_coin_game, algo::project::*, game::{strategy::VecStrat, dgame::{from_game::FromGame}}};

#[macro_use]
mod game;
mod games;
mod algo;
mod util;

fn main() {
    let g = GridPursuitGame::<3, 3, 2>::default();
    let g2 = DGame::<u32, 2>::from_game(&g, true).unwrap();

    println!("{:?}", g2);
    println!("{}, {}", g2.graph.node_count(), g2.graph.edge_count());

    let proj = Project(&g, 0);
    let proj2 = DGame::<u32, 1>::from_game(&proj, true).unwrap();
    println!("{:?}", proj2);
    println!("{}, {}", proj2.graph.node_count(), proj2.graph.edge_count());
}
