use crate::*;
use std::hash::Hash;

pub trait AbstractGame<const N: usize> {
    type Loc: Clone + Eq + Hash;
    type Obs: Clone + Eq + Hash;
    type Data;

    fn l0(&self) -> Self::Loc;
    fn n_actions(&self) -> [usize; N];
    fn obs(&self, loc: &Self::Loc) -> [Self::Obs; N];
    fn is_winning(&self, loc: &Self::Loc) -> bool;
    fn data(&self, loc: &Self::Loc) -> Self::Data;

    fn succ(
        &self,
        loc: &Self::Loc,
        f: impl FnMut([Act; N], Self::Loc)
    );

    fn build(&self) -> Game<Self::Data, N> {
        self.into()
    }
}
