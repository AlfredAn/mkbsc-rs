use std::rc::Rc;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::cell::RefCell;
use fixedbitset::FixedBitSet;
use std::hash::Hash;
use std::fmt::{Debug, self};
use derive_new::new;
use super::*;

pub trait AbstractGame<const N: usize>: Debug {
    type Loc: Clone + Eq + Hash + Debug;
    type Obs: Clone + Eq + Hash + Debug;
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

#[derive(new, Debug, Clone)]
pub struct Project<T, const N: usize> {
    g: Rc<Game<T, N>>,
    agt: Agt
}

impl<T: Debug + Clone, const N: usize> AbstractGame<1> for Project<T, N> {
    type Loc = game::Loc;
    type Obs = game::Obs;
    type Data = T;

    fn l0(&self) -> Self::Loc { 0 }
    fn n_actions(&self) -> [usize; 1] { [self.g.n_actions[self.agt]] }
    fn obs(&self, &l: &Self::Loc) -> [Self::Obs; 1] { [self.g.observe(l)[self.agt]] }
    fn is_winning(&self, &l: &Self::Loc) -> bool { self.g.is_winning(l) }
    fn data(&self, &l: &Self::Loc) -> T { self.g.data(l).clone() }

    fn succ(&self, &l: &Self::Loc, mut f: impl FnMut([Act; 1], Self::Loc)) {
        for (a, l2) in self.g.succ(l) {
            f([a[self.agt]], l2)
        }
    }
}

#[derive(new, Debug, Clone)]
pub struct KBSC<T> {
    g: Rc<Game<T, 1>>
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

#[derive(new, Clone)]
pub struct KBSCData<T> {
    g: Rc<Game<T, 1>>,
    s: ObsSubset
}

impl<T: fmt::Debug> fmt::Debug for KBSCData<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.s.iter(&self.g)
            .map(|l| self.g.data(l))
            .format_with("|", |x, f|
                f(&format_args!("{:?}", x))
            )
        )
    }
}

impl<T: fmt::Display> fmt::Display for KBSCData<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.s.iter(&self.g)
            .map(|l| self.g.data(l))
            .format("|")
        )
    }
}

thread_local!(
    static TEMP: RefCell<BTreeSet<(Act, Obs, Loc)>> = Default::default();
);

impl<T: Debug> AbstractGame<1> for KBSC<T> {
    type Loc = ObsSubset;
    type Obs = Self::Loc;
    type Data = KBSCData<T>;

    fn l0(&self) -> Self::Loc { ObsSubset::s0(&self.g) }
    fn n_actions(&self) -> [usize; 1] { self.g.n_actions }
    fn obs(&self, s: &Self::Loc) -> [Self::Obs; 1] { [s.clone()] }
    fn is_winning(&self, s: &Self::Loc) -> bool {
        s.iter(&self.g).all(|l| self.g.is_winning(l))
    }
    fn data(&self, l: &Self::Loc) -> Self::Data {
        KBSCData::new(self.g.clone(), l.clone())
    }

    fn succ(&self, s: &Self::Loc, mut f: impl FnMut([Act; 1], Self::Loc)) {
        let g = &self.g;
        
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
