#![allow(dead_code)]

pub use prelude::*;

mod prelude;

mod game;
mod algo;
mod util;
mod io;
mod cli;

fn main() -> anyhow::Result<()> {
    run_cli()?;
    //debug()?;
    Ok(())
}

fn run_cli() -> anyhow::Result<()> {
    let cli = cli::parse()?;
    cli::run(&cli)?;

    Ok(())
}

fn load_game() -> Game<2> {
    let g = include_game!("../games/cart_pushing", 2).build().game;
    let mut stack = MKBSCStack::new(g);
    for _ in 0..2 {
        stack.push();
    }
    let mut g = stack.last().game().as_ref().clone();
    g.origin = None;
    g
}

fn debug() -> anyhow::Result<()> {
    let g0 = load_game();
    println!("{}", g0);

    for _ in 0..50 {
        let g = load_game();
        println!("{}", g);
        println!("{}, {}", g.loc.len(), is_isomorphic(&g0, &g, true));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cart_pushing_game() {
        fn load_game(iters: u32) -> Game<2> {
            let g = include_game!("../games/cart_pushing", 2).build().game;
            let mut stack = MKBSCStack::new(g);
            for _ in 0..iters {
                stack.push();
            }
            let mut g = stack.last().game().as_ref().clone();
            g.origin = None;
            g
        }

        for iters in 0..6 {
            println!("\n--{} iterations--", iters);

            let g0 = load_game(iters);
            println!("{} locations", g0.loc.len());
    
            for _ in 0..50 {
                let g = load_game(iters);
                assert!(is_isomorphic(&g0, &g, true));
                print!(".");
            }
        }
    }
}
