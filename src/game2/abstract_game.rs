use itertools::Itertools;
use std::collections::BTreeSet;
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

    pub fn put<T>(&mut self, g: &Game<T, 1>, l: Loc) -> bool {
        assert_eq!(g.observe(l), [self.obs]);
        self.set.put(g.obs_offset(l)[0] as usize)
    }
}

thread_local!(
    static TEMP: RefCell<BTreeSet<(Act, Obs, Loc)>> = Default::default();
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
        
        TEMP.with(|r| {
            let mut succ = r.borrow_mut();
            succ.clear();

            for l in s.iter(g) {
                for ([a], l2) in g.succ(l) {
                    let [obs] = g.observe(l2);
                    succ.insert((a, obs, l2));
                }
            }

            for ((a, obs), group) in &succ.iter().group_by(|(a, obs, _)| (*a, *obs)) {
                let mut subset = ObsSubset::empty(g, obs);
                
                for (_, _, l) in group {
                    subset.put(g, *l);
                }
                f([a], subset);
            }
        });
    }
}
