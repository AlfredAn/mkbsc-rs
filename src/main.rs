#![feature(generic_associated_types)]

#![allow(unused_imports)]
use game::Game;
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
    let game = GridPursuitGame::<3, 3, 2>::default();
    let g = &game;

    let mut stack = vec![g.l0()];
    let mut visited = g.visit_map();
    while let Some(l) = stack.pop() {
        if visited.is_visited(&l) {
            continue;
        }
        visited.visit(l);

        print!("{}\n", l);
        for e in g.edges(l) {
            let a = g.act(e).collect_vec();
            if a.len() > 0 {
                //print!("{:?}\n{:?}\n\n", e.target(), a);
            }
            stack.push(e.target());
        }
    }
}
