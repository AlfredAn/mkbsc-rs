#![allow(dead_code)]
#![allow(unused_imports)]
use std::fmt::Debug;

use game::{Game, dgame::DGame};
use games::grid_pursuit::GridPursuitGame;
use itertools::Itertools;
use petgraph::visit::{Walker, Dfs, IntoNeighbors, IntoEdges, EdgeRef, Visitable, VisitMap};

use crate::{games::{three_coin_game, cup_game::cup_game}, algo::{project::*, kbsc::KBSC}, game::{strategy::VecStrat, dgame::{*, index::*}}};

#[macro_use]
mod game;
mod games;
mod algo;
mod util;

fn print_game<'a, G: Game<'a, N>, const N: usize>(g: &G) {
    let dg = DGame::<u32, N>::from_game(g, false).unwrap();
    println!("{:?}", dg);
}

fn main() {
    let g = cup_game().unwrap();
    print_game(&g);
    let proj = Project(g, agent_index(1));
    print_game(&proj);
    let kbsc = KBSC::new(proj);
    print_game(&kbsc);
}
