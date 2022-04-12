#![allow(dead_code)]

use std::rc::Rc;
use crate::game2::abstract_game::*;
use crate::game2::Project;
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
    let g = Rc::new(CupGame().build());
    println!("G: {:?}", g);

    let project0 = Rc::new(Project::new(g.clone(), 0).build());
    println!("G|0: {:?}", project0);

    let project1 = Rc::new(Project::new(g.clone(), 1).build());
    println!("G|1: {:?}", project1);

    let kbsc0 = Rc::new(KBSC::new(project0.clone()).build());
    println!("(G|0)^K: {:?}", kbsc0);

    let kbsc1 = Rc::new(KBSC::new(project1.clone()).build());
    println!("(G|1)^K: {:?}", kbsc1);
}
