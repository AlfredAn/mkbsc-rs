#![allow(dead_code)]
#![allow(unused_imports)]
use crate::game::GameRef;
use crate::game::Game1;
use crate::algo::strat_synth::strategy1::*;
use crate::from_game;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use petgraph::dot::*;
use std::fmt;
use std::io::Write;

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
#[macro_use]
mod macros;
mod test;

fn main() {
    strategy_synthesis_test();
}

fn strategy_synthesis_test() {
    let g = cup_game().mkbsc();
    
    println!("{:?}\n", g.dgame());

    let mut s = g.all_strategies();
    loop {
        println!("\n\n{:?}", s.get());
        if !s.advance() { break; }
    }
}
