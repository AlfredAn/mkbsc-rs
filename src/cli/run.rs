use anyhow::{Context, ensure};
use once_cell::unsync::OnceCell;

use crate::{*, io_game::*};

use super::{Action, PostProcessing};

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
}

#[enum_dispatch]
pub trait RunnerTrait {
    fn run(&mut self, action: &Action, post: &PostProcessing) -> anyhow::Result<()>;
}

impl<const N: usize> RunnerTrait for Runner<N> {
    fn run(&mut self, action: &Action, post: &PostProcessing) -> anyhow::Result<()> {
        match action {
            Action::None | Action::MKBSC(_) => {
                let g = match action {
                    Action::None => self.game().clone(),
                    Action::MKBSC(n) => self.mkbsc(*n).clone(),
                };

                if post.project.is_some() || post.kbsc {
                    let agt = if let Some(agt) = &post.project {
                        self.io_game.find_agent(agt)
                            .with_context(|| format!("undefined agent: {}", agt))?
                    } else {
                        ensure!(N == 1, "KBSC can only be performed on single-agent games");
                        0
                    };
                    
                    let gi = Project::new(g.clone(), agt).build();

                    if post.kbsc {
                        let gk = KBSC::new(gi.game).build();
                        println!("{:?}", gk);
                    } else {
                        println!("{:?}", gi);
                    }
                } else {
                    println!("{:?}", g);
                }
            }
        }
        Ok(())
    }
}

impl<const N: usize> Runner<N> {
    fn mkbsc(&mut self, iterations: usize) -> &Rc<Game<N>> {
        let mkbsc = self.stack();

        for i in 0..=iterations {
            // println!("-----G^({}K)-----", i);

            if mkbsc.len() <= i {
                mkbsc.push();
            }

            // println!("{:?}", mkbsc.get(i).game());
        }

        mkbsc.get(iterations).game()
    }


}
