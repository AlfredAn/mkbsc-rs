#![allow(dead_code)]

pub use prelude::*;

mod prelude;

mod game;
mod algo;
mod util;
mod io;
mod cli;

fn main() -> anyhow::Result<()> {
    //run_cli()?;
    debug()?;
    Ok(())
}

fn run_cli() -> anyhow::Result<()> {
    let cli = cli::parse()?;
    cli::run(&cli)?;

    Ok(())
}

fn load_game() -> Game<2> {
    let g = include_game!("../games/test/test_iso_3a", 2).build().game;
    let mut stack = MKBSCStack::new(g);
    for _ in 0..0 {
        stack.push();
    }
    let mut g = stack.last().game().as_ref().clone();
    //g.origin = None;
    g
}

fn debug() -> anyhow::Result<()> {
    for _ in 0..1 {
        let g = load_game();
        //let g2 = load_game();

        println!("{}", g);
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
    
            for _ in 0..50 {
                let g0 = load_game(iters);
                let g = load_game(iters);

                if iters >= 2 {
                    println!("{}\n{}\n\n\n", g0, g);
                }

                assert!(is_isomorphic(&g0, &g, false));
                print!(".");
            }
        }
    }
}
