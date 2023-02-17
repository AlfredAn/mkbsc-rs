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

mod test {
    use super::*;

    #[test]
    fn test_cart_pushing_game() {
        fn load_game() -> Game<2> {
            let g = include_game!("../games/cart_pushing", 2).build().game;
            let mut stack = MKBSCStack::new(g);
            for _ in 0..6 {
                stack.push();
            }
            let mut g = stack.last().game().as_ref().clone();
            g.origin = None;
            g
        }

        let g0 = load_game();
        println!("{}\n", g0.loc.len());

        for _ in 0..50 {
            let g = load_game();
            assert!(is_isomorphic(&g0, &g, true));
        }
    }
}