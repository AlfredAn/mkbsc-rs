use crate::*;

#[derive(new, Debug, Clone)]
pub struct Project<T, const N: usize> {
    g: Rc<Game<T, N>>,
    agt: Agt
}

impl<T: Clone, const N: usize> AbstractGame<1> for Project<T, N> {
    type Loc = game::Loc;
    type Obs = game::Obs;
    type Data = T;

    fn l0(&self) -> Self::Loc { 0 }
    fn n_actions(&self) -> [usize; 1] { [self.g.n_actions[self.agt]] }
    fn obs(&self, &l: &Self::Loc) -> [Self::Obs; 1] { [self.g.observe(l)[self.agt]] }
    fn is_winning(&self, &l: &Self::Loc) -> bool { self.g.is_winning(l) }
    fn data(&self, &l: &Self::Loc) -> Self::Data { self.g.data(l).clone() }

    fn succ(&self, &l: &Self::Loc, mut f: impl FnMut([Act; 1], Self::Loc)) {
        for &(a, l2) in self.g.successors(l) {
            f([a[self.agt]], l2)
        }
    }
}
