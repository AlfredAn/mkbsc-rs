#![allow(dead_code)]

pub use prelude::*;

#[macro_use]
mod prelude;

mod game;
mod algo;
mod util;

fn main() {
    let g = Rc::new(CupGame().build());

    let mkbsc = MKBSC::new(g.clone());
    let gk = Rc::new(mkbsc.build());

    println!("G: {:?}", g);

    println!("G|0: {:?}", mkbsc.gi[0]);
    println!("G|1: {:?}", mkbsc.gi[1]);

    println!("(G|0)^K: {:?}", mkbsc.gki[0]);
    println!("(G|1)^K: {:?}", mkbsc.gki[1]);

    println!("G^K: {:?}", gk);

    let mkbsc2 = MKBSC::new(gk);
    let g2k = mkbsc2.build();
    println!("G^(2K): {:?}", &g2k);

    let mut strategies = all_strategies(&mkbsc2);
    loop {
        println!("{:?}", strategies.get_ref());

        let winning = verify_strategy(&g2k, strategies.get_ref());
        println!("{:?}", winning);

        if !strategies.advance() { break; }
    }
}
