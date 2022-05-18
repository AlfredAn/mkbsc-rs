#![allow(dead_code)]

use std::time::SystemTime;

pub use prelude::*;

mod prelude;

mod game;
mod algo;
mod util;
mod io;
mod cli;

fn main() -> anyhow::Result<()> {
    run_cli()?;
    Ok(())
}

fn run_cli() -> anyhow::Result<()> {
    let cli = cli::parse()?;
    cli::run(&cli)?;

    Ok(())
}
