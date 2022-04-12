#![allow(dead_code)]

use crate::game2::Game;
use crate::game2::cup_game::CupGame;

#[macro_use]
mod game;
mod games;
mod algo;
#[macro_use]
mod util;
#[macro_use]
mod macros;
mod test;
mod game2;

fn main() {
    let g = CupGame();
    println!("{:?}", g);

    let g: Game<_, 2> = (&g).into();
    println!("{:?}", g);
}
