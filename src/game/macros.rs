use std::hash::Hash;
use crate::game::Game;

macro_rules! derive_ii {
    ($na:expr) => {
        fn obs_eq(&self, l1: &Self::Loc, l2: &Self::Loc, agt: Self::Agent) -> bool {
            l1 == l2
        }
    };
}

macro_rules! derive_ma {
    ($l:lifetime) => {
        type Agent = crate::game::dgame::index::ZeroIndex;
        fn actions_i<'z>(&'z self, _: Self::Agent) -> crate::util::Itr<'z, Self::Act>
        where
            $l: 'z
        {
            Box::new(self.actions().map(|[a]| a))
        }
    }
}

macro_rules! derive_magiian {
    () => {}
}

#[derive(Debug)]
pub struct Obs<'a, G: Game<'a, N>, const N: usize>(pub(crate) &'a G, pub(crate) G::Loc, pub(crate) G::Agent);

impl<'a, G: Game<'a, N>, const N: usize> PartialEq<Self> for Obs<'a, G, N> {
    fn eq(&self, o: &Self) -> bool {
        self.0.obs_eq(&self.1, &o.1, self.2)
    }
}

impl<'a, G: Game<'a, N>, const N: usize> Eq for Obs<'a, G, N> {}

impl<'a, G: Game<'a, N>, const N: usize> Clone for Obs<'a, G, N> {
    fn clone(&self) -> Self {
        *self
    }
}

pub(crate) use derive_ii;
pub(crate) use derive_ma;
pub(crate) use derive_magiian;
//pub(crate) use derive_observe;
