use crate::{*, io_game::*};

use super::{Action, MKBSCAction};

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
    pub fn new(io_game: IOGameEnum, preserve_origin: bool) -> Self {
        macro_rules! new {
            ($n:expr) => {
                Runner::<$n>::new(
                    io_game.try_into().unwrap(),
                    preserve_origin
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
    preserve_origin: bool
}

impl<const N: usize> Runner<N> {
    fn new(io_game: IOGame<N>, preserve_origin: bool) -> Self {
        Self {
            io_game: Rc::new(io_game),
            preserve_origin
        }
    }

    fn game(&self) -> Rc<Game<N>> {
        self.io_game.clone().build_ext(self.preserve_origin).game
    }
}

#[enum_dispatch]
pub trait RunnerTrait {
    fn run(&mut self, action: &Action) -> anyhow::Result<()>;
}

impl<const N: usize> RunnerTrait for Runner<N> {
    fn run(&mut self, action: &Action) -> anyhow::Result<()> {
        match action {
            Action::MKBSC(a) => self.mkbsc(a),
            Action::Synth => self.synthesize(true, false),
        }
        
        Ok(())
    }
}

impl<const N: usize> Runner<N> {
    fn mkbsc(&mut self, action: &MKBSCAction) {
        let mut g = self.game().clone();

        if action.print_iteration {
            println!("-----G^({}K)-----", 0);
        }
        if action.print_sizes {
            println!("n = {}", g.loc.len());
        }

        if action.print_games {
            println!("{}", display(|f| g.format(f, &action.format)));
        }

        for i in 0..action.iterations {
            if action.print_iteration {
                println!("-----G^({}K)-----", i+1);
            }

            let g_prev = g.clone();
            g = Rc::new(MKBSC::new(g))
                .build_ext(action.keep_structure)
                .game;

            if action.print_sizes {
                println!("n = {}", g.loc.len());
            }

            if action.print_games {
                println!("{}", display(|f| g.format(f, &action.format)));
            }

            if action.check_convergence && is_isomorphic(&g, &g_prev, true) {
                println!("isomorphic, stopping at G^({}K)", i);
                g = g_prev;
                break;
            }
        }

        if action.kbsc || action.project.is_some() {
            let agt = if let Some(agt) = &action.project {
                self.io_game.find_agent(agt).unwrap()
            } else {
                assert_eq!(N, 1);
                agt(0)
            };

            let mut g = Project::new(g, agt).build().game;

            if action.kbsc {
                g = KBSC::new(g).build().game;
            }

            if action.print_result {
                println!("{}", display(|f| g.format(f, &action.format)));
            }
        } else {
            if action.print_result && !action.print_games {
                println!("{}", display(|f| g.format(f, &action.format)));
            }
        }
    }

    fn synthesize(&mut self, print: bool, find_all: bool) {
        let mut stack = MKBSCStack::new(self.io_game.clone().build().game);
        let mut stats = Stats::default();

        for i in 0.. {
            if print { println!("-----G^({}K)-----", i); }

            if stack.len() < i+1 {
                if print { println!("applying MKBSC..."); }
                stack.push();
                if print { println!("n = {}", stack.last().game().n_loc()); }
                if i > 1 && is_isomorphic(stack.get(i).game(), stack.get(i-1).game(), true) {
                    if print { println!("isomorphic, stopping at G^({}K)", i-1); }
                    break;
                }
            } else {
                if print { println!("n = {}", stack.last().game().n_loc()); }
            }
            // if print { println!("{:?}", stack.last().game()); }
            
            if print { println!("finding strategy..."); }

            let (result, stats2) = stack.find_strategy(print, find_all);
            stats += stats2;

            if let Some(profile) = result {
                if print { println!("strategy found"); }
                if print { println!("{:?}", profile); }
                break;
            }

            if print { println!("no strategy found"); }
        }

        if print { println!("{:?}", stats); }
    }
}
