use crate::*;

pub trait AbstractGame<const N: usize> {
    type Loc: Clone + Eq + Hash;
    type Obs: Clone + Eq + Hash;

    fn l0(&self) -> Self::Loc;
    fn n_actions(&self) -> [usize; N];
    fn obs(&self, loc: &Self::Loc) -> [Self::Obs; N];
    fn is_winning(&self, loc: &Self::Loc) -> bool;

    fn succ(
        &self,
        loc: &Self::Loc,
        f: impl FnMut([Act; N], Self::Loc)
    );

    fn build(self) -> ConstructedGame<Self, N> where Self: Sized + 'static {
        Rc::new(self).build()
    }

    fn build_ext(self, keep_origin: bool) -> ConstructedGame<Self, N> where Self: Sized + 'static {
        Rc::new(self).build_ext(keep_origin)
    }

    fn fmt_loc(&self, f: &mut fmt::Formatter, l: &Self::Loc) -> fmt::Result;

    fn fmt_obs(&self, f: &mut fmt::Formatter, _agt: Agt, _o: &Self::Obs) -> fmt::Result {
        write!(f, "?")
    }
}

pub trait AbstractGameRc<G: AbstractGame<N> + 'static, const N: usize> {
    fn build(self) -> ConstructedGame<G, N>;
    fn build_ext(self, keep_origin: bool) -> ConstructedGame<G, N>;
}

impl<G: AbstractGame<N> + 'static, const N: usize> AbstractGameRc<G, N> for Rc<G> {
    fn build(self) -> ConstructedGame<G, N> {
        build_game(self, true)
    }

    fn build_ext(self, keep_origin: bool) -> ConstructedGame<G, N> {
        build_game(self, keep_origin)
    }
}
