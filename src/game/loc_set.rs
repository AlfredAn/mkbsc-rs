use derive_more::*;
use itertools::{izip};

use crate::{*, string::{Interned, Interner}};

#[derive(PartialEq, Eq, Hash, Debug, From, Into, Clone)]
pub struct LocSet {
    s: FixedBitSet
}

thread_local! {
    static GLOBAL_LOCSET: Interner<LocSet> = Interner::default();
}

impl LocSet {
    pub fn from_raw(len: usize, itr: impl Iterator<Item=u32>) -> Self {
        FixedBitSet::with_capacity_and_blocks(len, itr).into()
    }

    pub fn intern(&self) -> Interned<Self> {
        GLOBAL_LOCSET.with(|int|
            int.intern(self)
        )
    }

    pub fn new<const N: usize>(g: &Game<N>) -> Self {
        Self { s: FixedBitSet::with_capacity(g.n_loc()) }
    }

    pub fn singleton<const N: usize>(g: &Game<N>, l: Loc) -> Self {
        let mut result = Self::new(g);
        result.insert(l);
        result
    }

    pub fn from_iter<const N: usize>(g: &Game<N>, iter: impl IntoIterator<Item=Loc>) -> Self {
        let mut result = Self::new(g);
        result.extend(iter);
        result
    }

    pub fn raw(&self) -> &[u32] {
        self.s.as_slice()
    }

    pub fn iter(&self) -> impl Iterator<Item=Loc> + '_ {
        self.s.ones().map(|l| loc(l))
    }

    pub fn contains(&self, l: Loc) -> bool {
        self.s.contains(l.index())
    }

    pub fn is_empty(&self) -> bool {
        self.s.count_ones(..) == 0
    }

    pub fn insert(&mut self, l: Loc) {
        self.s.insert(l.index());
    }

    pub fn put(&mut self, l: Loc) -> bool {
        self.s.put(l.index())
    }

    pub fn remove(&mut self, l: Loc) {
        self.s.set(l.index(), false);
    }

    pub fn first(&self) -> Option<Loc> {
        self.iter().next()
    }
    
    pub fn clear(&mut self) {
        self.s.clear()
    }

    pub fn replace_with(&mut self, source: &LocSet) {
        assert_eq!(self.s.len(), source.s.len());
        for (b1, b2) in izip!(self.s.as_mut_slice(), source.raw()) {
            *b1 = *b2;
        }
    }

    pub fn intersection(s: &[&LocSet]) -> Self {
        let len = s.iter().map(|s| s.s.len()).min().unwrap();
        
        Self::from_raw(
            len,
            (0..(len+31)/32).map(|i|
                s.iter()
                    .map(|s| s.raw()[i])
                    .reduce(|a, b| a & b)
                    .unwrap()
            )
        )
    }

    pub fn intersect<const N: usize>(dest: &mut LocSet, s: [&LocSet; N]) {
        for i in 0..dest.raw().len() {
            dest.s.as_mut_slice()[i] = s.iter()
                .map(|s| s.raw()[i])
                .reduce(|a, b| a & b)
                .unwrap();
        }
    }
}

impl BitAndAssign<&Self> for LocSet {
    fn bitand_assign(&mut self, rhs: &Self) {
        assert_eq!(self.s.len(), rhs.s.len());
        self.s &= &rhs.s
    }
}

impl Extend<Loc> for LocSet {
    fn extend<I: IntoIterator<Item=Loc>>(&mut self, iter: I) {
        for l in iter {
            self.insert(l);
        }
    }
}
