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
                Project::new(self.base.clone(), agt(i)).build()
            )
        )
    }

    pub fn find_strategy(&mut self, print: bool, find_all: bool) -> (Option<&[Transducer; N]>, Stats) {
        let entry = self.last();
        let g = entry.game();
        
        if let Some(mkbsc) = entry.mkbsc() {
            if print { println!("starting strategy synthesis"); }

            let mut found_strategies = FxHashSet::default();

            let mut profile = None;
            let stats = find_strategy(
                mkbsc,
                |depth| {
                    if print { println!("depth: {depth}"); }
                    ControlFlow::Continue(())
                },
                |strat| {
                    if profile.is_none() {
                        profile = Some(strat.clone());
                    }
                    if !found_strategies.contains(strat) {
                        found_strategies.insert(strat.clone());
                        if print { println!("found strategy: {:#?}", strat); }
                    }
                    if find_all {
                        ControlFlow::Continue(())
                    } else {
                        ControlFlow::Break(())
                    }
                },
                find_all
            );

            if print { println!("number of strategies found: {}", found_strategies.len()); }

            if profile.is_none() { return (None, stats); }
            let profile = profile.unwrap();

            let transducer = match entry {
                StackElement::Base(_) => {
                    let s = array_init(|i|
                        self.base_proj()[i]
                            .translate_strategy(&profile[i])
                    );
                    let works = verify_strategy(g, &s);
                    assert!(works);
                    from_iter(s.into_iter().enumerate().map(|(i, si)|
                        si.transducer_ma(g, agt(i))
                    )).unwrap()
                },
                StackElement::MKBSC(g) => {
                    let profile = g.from_kbsc_profile(profile.ref_array());
                    let s = profile.ref_array();
                    let works = verify_strategy(g, &s);
                    assert!(works);
                    from_iter(s.into_iter().enumerate().map(|(i, si)|
                        si.transducer_ma(g, agt(i))
                    )).unwrap()
                }
            };

            assert!(verify_strategy(g, &transducer));

            self.strat.clear();
            self.strat.push(transducer);

            for i in (0..self.len()-1).rev() {
                let last_strat = self.strat.last().unwrap().ref_array();

                let last_g = self.get(i+1).mkbsc().unwrap();
                let g = self.get(i).game();

                let transducer = {
                    let translated = last_g.translate_strategy(last_strat);
                    array_init(|i|
                        translated[i].transducer_ma(g, agt(i))
                    )
                };

                assert!(verify_strategy(g, &transducer));

                self.strat.push(transducer);
            }

            self.strat.reverse();
            return (self.strat(0), stats);
        }

        (None, Stats::default())
    }
}
