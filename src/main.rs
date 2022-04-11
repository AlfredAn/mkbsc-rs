#![allow(dead_code)]

use crate::game::*;
use crate::games::cup_game::cup_game;

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
    strategy_synthesis_test();
}

fn strategy_synthesis_test() {
    let g = cup_game().mkbsc();
    
    println!("{:?}\n", g.dgame());

    let mut s = g.all_strategies();
    loop {
        println!("\n\n{:?}", s.get_raw());
        //println!("{:?}", verify_memoryless_strategy());

        if !s.advance() { break; }
    }
}
