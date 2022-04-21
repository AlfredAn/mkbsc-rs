use derive_more::*;

use crate::*;

#[derive(PartialEq, Eq, Hash, Debug, From, Into, Clone)]
pub struct LocSet {
    s: FixedBitSet
}

impl LocSet {
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

    pub fn from_subset(g: &Game<1>, s: &ObsSubset) -> Self {
        Self::from_iter(g, s.iter(g))
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

    pub fn first(&self) -> Option<Loc> {
        self.iter().next()
    }
    
    /*pub fn fmt_debug<T: Debug>
        (&self, f: &mut fmt::Formatter, d: &impl GameDataT<T>)
        -> fmt::Result {
        write!(f, "{}", self.iter()
            .map(|l| &d[l])
            .format_with("|", |x, f|
                f(&format_args!("{:?}", x))
            )
        )
    }

    pub fn fmt_display<T: Display>
        (&self, f: &mut fmt::Formatter, d: &impl GameDataT<T>)
        -> fmt::Result {
        write!(f, "{}", self.iter()
            .map(|l| &d[l])
            .format("|")
        )
    }*/
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
