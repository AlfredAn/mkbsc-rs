#![allow(dead_code)]

pub use prelude::*;

#[macro_use]
mod prelude;

mod game;
mod algo;
mod util;

fn main() {
    let g = CupGame().build();
    let mut stack = MKBSCStack::new(g.game);

    loop {
        println!("--{}--", stack.len() - 1);
        println!("{:?}", stack.last().game());

        let strat = stack.find_strategy();
        println!("{:?}\n", strat);

        if strat.is_some() {
            break;
        }

        stack.push();
    }
}
