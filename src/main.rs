#![allow(dead_code)]

use crate::game2::*;

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
    let g = Rc::new(CupGame().build());

    let mkbsc = MKBSC::new(g.clone());
    let gk = Rc::new(mkbsc.build());

    println!("G: {:?}", g);

    println!("G|0: {:?}", mkbsc.gi[0]);
    println!("G|1: {:?}", mkbsc.gi[1]);

    println!("(G|0)^K: {:?}", mkbsc.gki[0]);
    println!("(G|1)^K: {:?}", mkbsc.gki[1]);

    println!("G^K: {:?}", gk);

    let g2k = MKBSC::new(gk).build();
    println!("G^(2K): {:?}", g2k);
}
