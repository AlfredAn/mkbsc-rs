use std::borrow::BorrowMut;
use std::cell::RefCell;
use fixedbitset::FixedBitSet;
use std::marker::PhantomData;
use std::borrow::Borrow;
use std::hash::Hash;
use std::fmt::Debug;
use derive_new::new;
use super::*;

pub trait AbstractGame<const N: usize>: Debug {
    type Loc: Clone + Eq + Hash + Debug;
    type Obs: Clone + Eq + Hash + Debug;

    fn l0(&self) -> Self::Loc;
    fn n_actions(&self) -> [usize; N];
    fn obs(&self, loc: &Self::Loc) -> [Self::Obs; N];
    fn is_winning(&self, loc: &Self::Loc) -> bool;

    fn succ(
        &self,
        loc: &Self::Loc,
        f: impl FnMut([Act; N], Self::Loc)
    );
}
/*
#[derive(new, Debug, Clone, Copy)]
pub struct AsAbstract<T, R: Borrow<Game<T, N>>, const N: usize> {
    g: R,
    _t: PhantomData<T>
}

impl<T: Debug, R: Borrow<Game<T, N>> + Debug, const N: usize> AbstractGame<N> for AsAbstract<T, R, N> {
    type Loc = game::Loc;
    type Obs = game::Obs;

    fn l0(&self) -> Self::Loc { 0 }
    fn n_actions(&self) -> [usize; N] { self.g.borrow().n_actions }
    fn obs(&self, &l: &Self::Loc) -> [Self::Obs; N] { self.g.borrow().observe(l) }
    fn is_winning(&self, &l: &Self::Loc) -> bool { self.g.borrow().is_winning(l) }

    fn succ(&self, &l: &Self::Loc, mut f: impl FnMut([Act; N], Self::Loc)) {
        for (a, l2) in self.g.borrow().succ(l) {
            f(a, l2)
        }
    }
}

#[derive(new, Debug, Clone, Copy)]
pub struct Project<T, R: Borrow<Game<T, N>>, const N: usize> {
    g: R,
    agt: Agt,
    _t: PhantomData<T>
}

impl<T: Debug, R: Borrow<Game<T, N>> + Debug, const N: usize> AbstractGame<1> for Project<T, R, N> {
    type Loc = game::Loc;
    type Obs = game::Obs;

    fn l0(&self) -> Self::Loc { 0 }
    fn n_actions(&self) -> [usize; 1] { [self.g.borrow().n_actions[self.agt]] }
    fn obs(&self, &l: &Self::Loc) -> [Self::Obs; 1] { [self.g.borrow().observe(l)[self.agt]] }
    fn is_winning(&self, &l: &Self::Loc) -> bool { self.g.borrow().is_winning(l) }

    fn succ(&self, &l: &Self::Loc, mut f: impl FnMut([Act; 1], Self::Loc)) {
        for (a, l2) in self.g.borrow().succ(l) {
            f([a[self.agt]], l2)
        }
    }
}

#[derive(new, Debug, Clone, Copy)]
pub struct KBSC<T, R: Borrow<Game<T, 1>>> {
    g: R,
    _t: PhantomData<T>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObsSubset {
    obs: game::Obs,
    set: FixedBitSet
}

impl ObsSubset {
    pub fn reset<T>(&mut self, g: &Game<T, 1>, obs: Obs) {
        self.obs = obs;
        self.set.clear();
        self.set.grow(g.obs_set(0, obs).len());
    }
    pub fn empty<T>(g: &Game<T, 1>, obs: Obs) -> Self {
        Self {
            obs,
            set: FixedBitSet::with_capacity(g.obs_set(0, obs).len())
        }
    }
    pub fn s0<T>(g: &Game<T, 1>) -> Self {
        let ([obs], [off]) = (g.observe(0), g.obs_offset(0));
        let mut result = Self::empty(g, obs);
        result.set.put(off as usize);
        result
    }
    pub fn iter<'a, T>(&'a self, g: &'a Game<T, 1>) -> impl Iterator<Item=Loc> + 'a {
        self.set.ones()
            .map(|i| g.obs_set(0, self.obs)[i])
    }
}

thread_local!(
    static OBS_SUBSET: RefCell<Vec<ObsSubset>> = Default::default();
);

impl<T: Debug, R: Borrow<Game<T, 1>> + Debug> AbstractGame<1> for KBSC<T, R> {
    type Loc = ObsSubset;
    type Obs = Self::Loc;

    fn l0(&self) -> Self::Loc { ObsSubset::s0(self.g.borrow()) }
    fn n_actions(&self) -> [usize; 1] { self.g.borrow().n_actions }
    fn obs(&self, s: &Self::Loc) -> [Self::Obs; 1] { [s.clone()] }
    fn is_winning(&self, s: &Self::Loc) -> bool {
        let g = self.g.borrow();
        s.iter(g).all(|l| g.is_winning(l))
    }

    fn succ(&self, s: &Self::Loc, mut f: impl FnMut([Act; 1], Self::Loc)) {
        let g = self.g.borrow();
        let g_succ = s.iter(g)
            .flat_map(|l|
                g.succ(l)
            );
        
        OBS_SUBSET.with(|r| {
            let mut r = r.borrow_mut();
            for (i, s) in r.iter_mut().enumerate() {
                s.reset(g, i as Obs);
            }
            while r.len() < g.n_obs(0) {
                let obs = r.len() as Obs;
                r.push(ObsSubset::empty(g, obs));
            }
        });
    }
}
*/