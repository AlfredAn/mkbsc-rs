#![allow(dead_code)]

use std::{fs, time::SystemTime};

use game::grid_pursuit::GridPursuitGame;
pub use prelude::*;

#[macro_use]
mod prelude;

mod game;
mod algo;
mod util;
mod io;
mod cli;

fn main() -> anyhow::Result<()> {
    // test_grid_pursuit()?;

    let before = SystemTime::now();
    run_cli()?;
    let elapsed = before.elapsed()?;

    println!("program: {:?}", elapsed);

    Ok(())
}

fn run_cli() -> anyhow::Result<()> {
    let cli = cli::parse();
    cli::run(&cli)?;

    Ok(())
}

fn test_grid_pursuit() -> anyhow::Result<()> {
    let g = GridPursuitGame::<2>::new(3, 3).build().game;

    // println!("{}", g);
    println!("{}", g.n_loc());
    println!("{}", g.n_obs(agt(0)));
    println!("{}", g.n_obs(agt(1)));

    // g = if true {
    //     let g: IOGame<2> = IOGameEnum::new(
    //         parse(&g.to_string()).unwrap()
    //     ).unwrap()
    //         .try_into().unwrap();
    //     g.build().game
    // } else {
    //     g
    // };

    // // println!("{}", g);
    // println!("{}", g.n_loc());
    // println!("{}", g.n_obs(0));
    // println!("{}", g.n_obs(1));

    // let gk = MKBSC::new(g.game.clone()).build();
    // println!("{}", gk.n_loc());
    
    let data = g.to_string();
    fs::write("games/gp3", data)?;

    Ok(())
}
