use crate::*;

#[derive(new, Debug, Clone)]
pub struct Project<const N: usize> {
    g: Rc<Game<N>>,
    agt: Agt
}

impl<const N: usize> AbstractGame<1> for Project<N> {
    type Loc = Loc;
    type Obs = Obs;

    fn l0(&self) -> Loc { self.g.l0() }
    fn n_actions(&self) -> [usize; 1] { [self.g.n_actions[self.agt.index()]] }
    fn obs(&self, &l: &Loc) -> [Obs; 1] { [self.g.observe(l)[self.agt.index()]] }
    fn is_winning(&self, &l: &Loc) -> bool { self.g.is_winning(l) }

    fn succ(&self, &l: &Loc, mut f: impl FnMut([Act; 1], Loc)) {
        for &(a, l2) in self.g.successors(l) {
            f([a[self.agt.index()]], l2)
        }
    }
    
    fn fmt_loc(&self, f: &mut fmt::Formatter, &l: &Self::Loc) -> fmt::Result {
        self.g.fmt_loc(f, l)
    }
}

impl<const N: usize> ConstructedGame<Project<N>, 1> {
    pub fn translate_strategy(
        &self,
        strat: impl Strategy
    ) -> impl Strategy {
        proj_to_base_strategy(self.clone(), strat)
    }
}

