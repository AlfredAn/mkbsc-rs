use once_cell::unsync::OnceCell;

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
    pub fn new(io_game: IOGameEnum) -> Self {
        macro_rules! new {
            ($n:expr, $g:expr) => {
                Runner::<$n>::new(
                    $g.try_into().unwrap(),
                ).into()
            }
        }

        match io_game.n_agents() {
            1 => new!(1, io_game),
            2 => new!(2, io_game),
            3 => new!(3, io_game),
            4 => new!(4, io_game),
            5 => new!(5, io_game),
            6 => new!(6, io_game),
            7 => new!(7, io_game),
            8 => new!(8, io_game),
            _ => panic!("unsupported number of agents: {}", io_game.n_agents())
        }
    }
}

#[derive(Debug)]
pub struct Runner<const N: usize> {
    io_game: Rc<IOGame<N>>,
    game: OnceCell<ConstructedGame<IOGame<N>, N>>,
    stack: Option<MKBSCStack<N>>
}

impl<const N: usize> Runner<N> {
    fn new(io_game: IOGame<N>) -> Self {
        Self {
            io_game: Rc::new(io_game),
            game: Default::default(),
            stack: None
        }
    }

    fn game(&self) -> &Rc<Game<N>> {
        self.game.get_or_init(|| self.io_game.clone().build())
    }

    fn stack(&mut self) -> &mut MKBSCStack<N> {
        if self.stack.is_none() {
            self.stack = Some(
                MKBSCStack::new(
                    self.game().clone()
                )
            );
        }
        self.stack.as_mut().unwrap()        
    }

    fn clear_stack(&mut self) {
        if let Some(stack) = &mut self.stack {
            stack.clear()
        }
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
            Action::Synth(_) => self.synthesize(),
        }
        
        Ok(())
    }
}

impl<const N: usize> Runner<N> {
    fn mkbsc(&mut self, action: &MKBSCAction) {
        self.clear_stack();
        let mut g = self.game().clone();

        for i in 0..action.iterations {
            if action.print_iteration {
                println!("-----G^({}K)-----", i+1);
            }

            let g_prev = g.clone();
            if action.keep_structure {
                g = Rc::new(MKBSC::new(g))
                    .build()
                    .game;
            } else {
                g = Rc::new(MKBSC::new(g))
                    .build_no_origin();
            }

            if action.print_sizes {
                println!("n = {}", g.loc.len());
            }

            if action.print_games {
                println!("{:?}", g);
            }

            if action.check_convergence && is_isomorphic(&g, &g_prev, true) {
                println!("isomorphic, stopping at G^({}K)", i);
                g = g_prev;
                break;
            }
        }

        if action.print_result && !action.print_games {
            println!("{:?}", g);
        }
    }

    fn synthesize(&mut self) {
        self.clear_stack();
        let stack = self.stack();

        for i in 0.. {
            println!("-----G^({}K)-----", i);

            if stack.len() < i+1 {
                println!("applying MKBSC...");
                stack.push();
                println!("n = {}", stack.last().game().n_loc());
                if is_isomorphic(stack.get(i).game(), stack.get(i-1).game(), true) {
                    println!("isomorphic, stopping at G^({}K)", i-1);
                    break;
                }
            } else {
                println!("n = {}", stack.last().game().n_loc());
            }

            
            println!("finding strategy...");
            if let Some(profile) = stack.find_strategy() {
                println!("strategy found");
                println!("{:?}", profile);
                break;
            }

            println!("no strategy found");
        }
    }
}
