#![allow(dead_code)]
#![allow(unused_imports)]
use std::fmt::Debug;

use algo::mkbsc::MKBSC;
use game::{Game, dgame::DGame};
use games::grid_pursuit::GridPursuitGame;
use itertools::Itertools;
use petgraph::visit::{Walker, Dfs, IntoNeighbors, IntoEdges, EdgeRef, Visitable, VisitMap};

use crate::{games::{three_coin_game, cup_game::cup_game}, algo::{project::*, kbsc::KBSC}, game::{strategy::VecStrat, dgame::{*, index::*}}};

#[macro_use]
mod game;
mod games;
mod algo;
#[macro_use]
mod util;

fn print_game<'a, G: Game<'a, N>, const N: usize>(g: G) {
    let dg = DGame::<u32, N>::from_game(g, false).unwrap();
    println!("{:?}", dg);
}

fn main() {
    let g = cup_game().unwrap();
    print_game(&g);
    let proj = Project(g.clone(), agent_index(1));
    print_game(&proj);
    let kbsc = KBSC::new(proj);
    print_game(&kbsc);
    let mkbsc = MKBSC::new(g);
    print_game(&mkbsc);
    let mkbsc2 = MKBSC::new(mkbsc);
    print_game(&mkbsc2);
    let mkbsc3 = MKBSC::new(mkbsc2);
    print_game(&mkbsc3);
}
