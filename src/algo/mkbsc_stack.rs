use derive_more::*;
use once_cell::unsync::OnceCell;
use crate::*;

type G<const N: usize> = Rc<Game<N>>;
type GK<const N: usize> = ConstructedGame<MKBSC<N>, N>;

#[derive(Debug, Clone, Copy, From)]
pub enum StackElement<'a, const N: usize> {
    Base(&'a G<N>),
    MKBSC(&'a GK<N>)
}

impl<'a, const N: usize> StackElement<'a, N> {
    pub fn game(self) -> &'a G<N> {
        match self {
            StackElement::Base(g) => g,
            StackElement::MKBSC(g) => &g.game
        }
    }

    pub fn mkbsc(self) -> Option<&'a GK<N>> {
        match self {
            StackElement::Base(_) => None,
            StackElement::MKBSC(g) => Some(g)
        }
    }
}

#[derive(Debug)]
pub struct MKBSCStack<const N: usize> {
    pub base: G<N>,
    stack: Vec<GK<N>>,
    strat: Vec<[Transducer; N]>,

    base_proj: OnceCell<[ConstructedGame<Project<N>, 1>; N]>
}

impl<const N: usize> MKBSCStack<N> {
    pub fn new(base: G<N>) -> Self {
        Self {
            base,
            stack: Vec::new(),
            strat: Vec::new(),
            base_proj: OnceCell::new()
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len() + 1
    }

    pub fn get(&self, i: usize) -> StackElement<N> {
        assert!(i < self.len(), "index out of bounds: {} >= {}", i, self.len());
        if i == 0 {
            StackElement::Base(&self.base)
        } else {
            StackElement::MKBSC(&self.stack[i-1])
        }
    }

    pub fn last(&self) -> StackElement<N> {
        self.get(self.len() - 1)
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.strat.clear();
    }

    pub fn push(&mut self) -> &GK<N> {
        let g = self.last().game();
        let gk = MKBSC::new(g.clone()).build();
        self.stack.push(gk);
        self.last().mkbsc().unwrap()
    }

    pub fn parts<'a>(&'a self, entry: &'a StackElement<N>) -> [&'a G<1>; N] {
        match entry {
            StackElement::Base(_) => {
                array_init(|i|
                    &self.base_proj()[i].game
                )
            },
            StackElement::MKBSC(g) => {
                array_init(|i|
                    &g.origin().gki[i].game
                )
            }
        }
    }

    pub fn strat(&self, level: usize) -> Option<&[Transducer; N]> {
        self.strat.get(level)
    }

    fn base_proj(&self) -> &[ConstructedGame<Project<N>, 1>; N] {
        self.base_proj.get_or_init(||
            array_init(move |i|
                Project::new(self.base.clone(), i).build()
            )
        )
    }

    pub fn find_strategy(&mut self) -> Option<&[Transducer; N]> {
        let entry = self.last();
        let g = entry.game();
        let parts = self.parts(&entry).map(|rc| &**rc);

        for profile in all_strategies(parts).into_iter() {
            if verify_strategy(g, &profile).is_ok() {
                let profile = array_init(|i| &profile[i]);

                let transducer = array_init(|i|
                    match entry {
                        StackElement::Base(_) => {
                            self.base_proj()[i]
                                .translate_strategy(profile[i])
                                .transducer_ma(g, i)
                        },
                        StackElement::MKBSC(g) => {
                            g.from_kbsc_profile(profile)[i]
                                .transducer_ma(g, i)
                        },
                    }
                );
                self.strat.clear();
                self.strat.push(transducer);

                for i in (0..N-1).rev() {
                    let last_strat = self.strat.last().unwrap();
                    let last_strat = array_init(|i| &last_strat[i]);

                    let last_g = self.get(i+1).mkbsc().unwrap();
                    let g = self.get(i).game();

                    let transducer = array_init(|i|
                        last_g.translate_strategy(last_strat)[i]
                            .transducer_ma(g, i)
                    );

                    self.strat.push(transducer);
                }

                self.strat.reverse();
                return self.strat(0);
            }
        }

        None
    }
}
