use std::io::Write;

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

    base_proj: OnceCell<[ConstructedGame<Project<N>, 1>; N]>
}

impl<const N: usize> MKBSCStack<N> {
    pub fn new(base: G<N>) -> Self {
        Self {
            base,
            stack: Vec::new(),
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

    pub fn projection(&self, i: usize, agt: Agt) -> &ConstructedGame<Project<N>, 1> {
        &self.mkbsc(i+1)
            .origin().gi[agt.index()]
    }

    pub fn kbsc(&self, i: usize, agt: Agt) -> &ConstructedGame<KBSC, 1> {
        &self.mkbsc(i+1)
            .origin().gki[agt.index()]
    }

    pub fn mkbsc(&self, i: usize) -> &ConstructedGame<MKBSC<N>, N> {
        &self.get(i).mkbsc().unwrap()
    }

    pub fn clear(&mut self) {
        self.stack.clear();
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

    fn base_proj(&self) -> &[ConstructedGame<Project<N>, 1>; N] {
        self.base_proj.get_or_init(||
            array_init(move |i|
                Project::new(self.base.clone(), agt(i)).build()
            )
        )
    }

    fn translate_strategy(&self, profile: &[Strat; N], output: &mut (impl Write + ?Sized)) -> anyhow::Result<[Transducer; N]> {
        let mkbsc = self.last().mkbsc().unwrap();

        assert!(verify_strategy(&mkbsc.game, &mkbsc.from_kbsc_profile(profile.clone())));

        let transducers = profile.iter()
            .enumerate()
            .map(|(i, s)| {
                let translated = translate_strategy(self, agt(i), s.clone());
                let transducer = translated.transducer_ma(self.base.clone(), agt(i));
                transducer
            })
            .collect_array()
            .unwrap();

        assert!(verify_strategy(&*self.base, &transducers));

        writeln!(output, "{:?}", transducers)?;
        output.flush()?;

        Ok(transducers)
    }

    pub fn find_strategy_profile(&self, find_all: bool, output: &mut (impl Write + ?Sized), print_text: bool)
    -> anyhow::Result<(impl Iterator<Item=[Transducer; N]> + ExactSizeIterator, Stats)> {
        let mut stats = Stats::default();
        let mut found_strategies = FxHashMap::default();

        let entry = self.last();
        
        if let Some(mkbsc) = entry.mkbsc() {
            if print_text { println!("starting strategy synthesis"); }

            let mut error = None;
            stats += find_strategy(
                mkbsc,
                |depth| {
                    if print_text { println!("depth: {depth}"); }
                    ControlFlow::Continue(())
                },
                |strat| {
                    if !found_strategies.contains_key(strat) {
                        if print_text { println!("found strategy #{}", found_strategies.len()+1); }

                        match self.translate_strategy(strat, output) {
                            Ok(translated) => {
                                found_strategies.insert(strat.clone(), translated);

                                if find_all {
                                    ControlFlow::Continue(())
                                } else {
                                    ControlFlow::Break(())
                                }
                            },
                            Err(err) => {
                                error = Some(err);
                                ControlFlow::Break(())
                            },
                        }
                    } else {
                        ControlFlow::Continue(())
                    }
                },
                find_all
            );
            if let Some(error) = error {
                return Err(error);
            }
        } else {
            if print_text { println!("strategy synthesis for G^(0K) is not yet supported"); }
        }

        Ok((found_strategies.into_values(), stats))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::sink;

    #[test]
    fn test_cup_game() -> anyhow::Result<()> {
        let g = include_game!("../../games/cup_game", 2)
            .build().game;
        let mut stack = MKBSCStack::new(g);

        assert!(stack.find_strategy_profile(false, &mut sink(), false)?.0.next().is_none());
        assert!(stack.find_strategy_profile(true, &mut sink(), false)?.0.next().is_none());

        stack.push();

        assert!(stack.find_strategy_profile(false, &mut sink(), false)?.0.next().is_none());
        assert!(stack.find_strategy_profile(true, &mut sink(), false)?.0.next().is_none());

        stack.push();

        assert!(stack.find_strategy_profile(false, &mut sink(), false)?.0.exactly_one().is_ok());
        assert!(stack.find_strategy_profile(true, &mut sink(), false)?.0.exactly_one().is_ok());

        stack.push();

        assert!(stack.find_strategy_profile(false, &mut sink(), false)?.0.exactly_one().is_ok());
        assert!(stack.find_strategy_profile(true, &mut sink(), false)?.0.exactly_one().is_ok());

        Ok(())
    }
}
