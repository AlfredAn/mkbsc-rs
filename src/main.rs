#![feature(generic_associated_types)]

use crate::{games::three_coin_game, algo::count_locations};

mod game;
mod games;
mod algo;

fn main() {
    let g = three_coin_game();
    print!("{:#?}\n", g);
    print!("{}", count_locations(&g));
}
