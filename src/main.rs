#![allow(dead_code)]

pub use prelude::*;

#[macro_use]
mod prelude;

mod game;
mod algo;
mod util;
mod io;
mod cli;

fn main() -> anyhow::Result<()> {
    let cli = cli::parse();
    println!("{:?}", cli);
    cli::run(&cli)?;
    
    Ok(())
}
