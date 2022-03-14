#![feature(generic_associated_types)]

#![allow(unused_imports)]
use game::{Game, dgame::DMAGIIAN};
use games::grid_pursuit::GridPursuitGame;
use itertools::Itertools;
use petgraph::visit::{Walker, Dfs, IntoNeighbors, IntoEdges, EdgeRef, Visitable, VisitMap};

use crate::{games::three_coin_game, algo::{count_locations, play::{Play, until_win}, for_each_location}, game::{strategy::VecStrat}};

#[macro_use]
mod game;
mod games;
mod algo;
mod util;

fn main() {
    let g = GridPursuitGame::<5, 5, 2>::default();
    let g2 = DMAGIIAN::<u32, 2>::from_game(&g, true).unwrap();

    //print!("{:?}", g2);
    println!("{}, {}", g2.graph().node_count(), g2.graph().edge_count());
}
