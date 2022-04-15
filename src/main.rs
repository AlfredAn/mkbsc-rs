#![allow(dead_code)]

pub use prelude::*;

#[macro_use]
mod prelude;

mod game;
mod algo;
mod util;

fn main() {
    let g = CupGame().build();

    let gk = MKBSC::new(&g).build();

    println!("G: {:?}", g);

    println!("G|0: {:?}", gk.origin.gi[0]);
    println!("G|1: {:?}", gk.origin.gi[1]);

    println!("(G|0)^K: {:?}", gk.origin.gki[0]);
    println!("(G|1)^K: {:?}", gk.origin.gki[1]);

    println!("G^K: {:?}", gk);

    let g2k = MKBSC::new(&gk).build();
    println!("G^(2K): {:?}", &g2k);

    let mut strategies = all_strategies(&g2k.origin);
    loop {
        println!("{:?}", strategies.get_ref());

        let winning = verify_strategy(&g2k, strategies.get_ref());
        println!("{:?}", winning);

        if !strategies.advance() { break; }
    }
}
