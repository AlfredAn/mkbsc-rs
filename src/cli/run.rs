use std::io::Write;

use anyhow::ensure;

use crate::{*, io_game::*};

use super::{Action, TransformAction, SynthesizeAction, Verbosity};

#[derive(Debug)]
#[enum_dispatch(RunnerTrait)]
pub enum RunnerEnum {
    R1(Runner<1>),
    R2(Runner<2>),
    R3(Runner<3>),
    R4(Runner<4>),
    R5(Runner<5>),
    R6(Runner<6>),
    R7(Runner<7>),
    R8(Runner<8>)
}

impl RunnerEnum {
    pub fn new(io_game: IOGameEnum, verbosity: Verbosity) -> Self {
        macro_rules! new {
            ($n:expr) => {
                Runner::<$n>::new(
                    io_game.try_into().unwrap(),
                    verbosity
                ).into()
            }
        }

        match io_game.n_agents() {
            1 => new!(1),
            2 => new!(2),
            3 => new!(3),
            4 => new!(4),
            5 => new!(5),
            6 => new!(6),
            7 => new!(7),
            8 => new!(8),
            _ => panic!("unsupported number of agents: {}", io_game.n_agents())
        }
    }
}

#[derive(Debug)]
pub struct Runner<const N: usize> {
    io_game: Rc<IOGame<N>>,
    verbosity: Verbosity
}

impl<const N: usize> Runner<N> {
    fn new(io_game: IOGame<N>, verbosity: Verbosity) -> Self {
        Self {
            io_game: Rc::new(io_game),
            verbosity
        }
    }

    fn game(&self, preserve_origin: bool) -> Rc<Game<N>> {
        self.io_game.clone().build_ext(preserve_origin).game
    }

    fn is_quiet(&self) -> bool {
        self.verbosity.is_quiet()
    }

    fn is_normal(&self) -> bool {
        self.verbosity.is_normal()
    }

    fn is_verbose(&self) -> bool {
        self.verbosity.is_verbose()
    }
}

#[enum_dispatch]
pub trait RunnerTrait {
    fn run(&self, action: &Action, output: &mut (impl Write + ?Sized)) -> anyhow::Result<()>;
}

impl<const N: usize> RunnerTrait for Runner<N> {
    fn run(&self, a: &Action, output: &mut (impl Write + ?Sized)) -> anyhow::Result<()> {
        match a {
            Action::Transform(a) => self.mkbsc(a, output),
            Action::Synthesize(a) => self.synthesize(a)
        }
    }
}

impl<const N: usize> Runner<N> {
    fn mkbsc(&self, a: &TransformAction, output: &mut (impl Write + ?Sized)) -> anyhow::Result<()> {
        let mut g = self.game(a.keep_structure).clone();

        macro_rules! output {
            ($g:expr) => {{
                let g = $g;
                writeln!(output, "{}", display(|f| g.format(f, &a.output_format)))?;
                output.flush()?;
            }};
        }

        if !self.is_quiet() {
            println!("-----G^({}K)-----", 0);
            println!("n = {}", g.loc.len());
            if self.is_verbose() {
                output!(&g);
            }
        }

        for i in 0..a.max_iterations {
            if !self.is_quiet() {
                println!("-----G^({}K)-----", i+1);
            }

            let g_prev = g.clone();
            g = Rc::new(MKBSC::new(g))
                .build_ext(a.keep_structure)
                .game;

            if !self.is_quiet() {
                println!("n = {}", g.loc.len());
                if self.is_verbose() {
                    output!(&g);
                }
            }

            if a.check_iso && is_isomorphic(&g, &g_prev, true) {
                println!("isomorphic, stopping at G^({}K)", i);
                g = g_prev;
                break;
            }
        }

        if a.kbsc || a.project.is_some() {
            let agt = if let Some(agt) = &a.project {
                self.io_game.find_agent(agt).unwrap()
            } else {
                ensure!(N == 1, "KBSC requires a single-agent game");
                agt(0)
            };

            let mut g = Project::new(g, agt).build().game;

            if a.kbsc {
                g = KBSC::new(g).build().game;
            }

            output!(&g);
        } else {
            if !self.is_verbose() {
                output!(&g);
            }
        }

        Ok(())
    }

    fn synthesize(&self, a: &SynthesizeAction) -> anyhow::Result<()> {
        let mut stack = MKBSCStack::new(self.io_game.clone().build().game);
        let mut stats = Stats::default();

        for i in 0.. {
            if !self.is_quiet() { println!("-----G^({}K)-----", i); }

            if stack.len() < i+1 {
                if !self.is_quiet() { println!("applying MKBSC..."); }

                stack.push();

                if !self.is_quiet() { println!("n = {}", stack.last().game().n_loc()); }

                if i > 1 && is_isomorphic(stack.get(i).game(), stack.get(i-1).game(), true) {
                    if !self.is_quiet() { println!("isomorphic, stopping at G^({}K)", i-1); }
                    break;
                }
            } else {
                if !self.is_quiet() { println!("n = {}", stack.last().game().n_loc()); }
            }

            let (result, stats2) = stack.find_strategy_profile(a.find_all, true, !self.is_quiet());
            stats += stats2;

            if result.len() > 0 {
                if !self.is_quiet() {
                    println!("terminating search\n{} {} found",
                        result.len(),
                        if result.len() == 1 { "strategy" } else { "strategies" }
                    );
                }
                break;
            }

            if !self.is_quiet() { println!("no strategy found"); }

            if i as u64 >= a.max_iterations {
                if !self.is_quiet() { println!("reached limit, stopping"); }
                break;
            }
        }

        if !self.is_quiet() { println!("{:?}", stats); }

        Ok(())
    }
}
