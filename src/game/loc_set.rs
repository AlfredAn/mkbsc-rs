use std::marker::PhantomData;

use derive_more::*;

use crate::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug, From, Into)]
pub struct LocSet<T> {
    s: FixedBitSet,
    t_: PhantomData<T>
}

impl<T> LocSet<T> {
    pub fn new<const N: usize>(g: &Game<T, N>) -> Self {
        Self { s: FixedBitSet::with_capacity(g.n_loc()), t_: Default::default() }
    }

    pub fn from_iter<const N: usize>(g: &Game<T, N>, iter: impl IntoIterator<Item=Loc<T>>) -> Self {
        let mut result = Self::new(g);
        result.extend(iter);
        result
    }

    pub fn from_subset(g: &Game<T, 1>, s: &ObsSubset<T>) -> Self {
        Self::from_iter(g, s.iter(g))
    }

    pub fn iter(&self) -> impl Iterator<Item=Loc<T>> + '_ {
        self.s.ones().map(|l| loc(l))
    }

    pub fn contains(&self, l: Loc<T>) -> bool {
        self.s.contains(l.index())
    }

    pub fn is_empty(&self) -> bool {
        self.s.count_ones(..) == 0
    }

    pub fn insert(&mut self, l: Loc<T>) {
        self.s.insert(l.index());
    }

    pub fn put(&mut self, l: Loc<T>) -> bool {
        self.s.put(l.index())
    }
    
    pub fn fmt_debug(&self, g: &Game<T, 1>, f: &mut fmt::Formatter) -> fmt::Result
    where T: Debug {
        write!(f, "{}", self.iter()
            .map(|l| g.data(l))
            .format_with("|", |x, f|
                f(&format_args!("{:?}", x))
            )
        )
    }

    pub fn fmt_display(&self, g: &Game<T, 1>, f: &mut fmt::Formatter) -> fmt::Result
    where T: Display {
        write!(f, "{}", self.iter()
            .map(|l| g.data(l))
            .format("|")
        )
    }
}

impl<T> BitAndAssign<&Self> for LocSet<T> {
    fn bitand_assign(&mut self, rhs: &Self) {
        assert_eq!(self.s.len(), rhs.s.len());
        self.s &= &rhs.s
    }
}

impl<T> Extend<Loc<T>> for LocSet<T> {
    fn extend<I: IntoIterator<Item=Loc<T>>>(&mut self, iter: I) {
        for l in iter {
            self.insert(l);
        }
    }
}
