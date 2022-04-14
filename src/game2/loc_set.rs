use super::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug, From, Into)]
pub struct LocSet {
    s: FixedBitSet
}

impl LocSet {
    pub fn new<T, const N: usize>(g: &Game<T, N>) -> Self {
        Self { s: FixedBitSet::with_capacity(g.n_loc()) }
    }

    pub fn from_iter<T, const N: usize>(g: &Game<T, N>, iter: impl IntoIterator<Item=Loc>) -> Self {
        let mut result = Self::new(g);
        result.extend(iter);
        result
    }

    pub fn from_subset<T>(g: &Game<T, 1>, s: &ObsSubset) -> Self {
        Self::from_iter(g, s.iter(g))
    }

    pub fn iter(&self) -> impl Iterator<Item=Loc> + '_ {
        self.s.ones().map(|l| l as Loc)
    }

    pub fn is_empty(&self) -> bool {
        self.s.count_ones(..) == 0
    }

    pub fn insert(&mut self, l: Loc) {
        self.s.insert(l as usize);
    }

    pub fn put(&mut self, l: Loc) -> bool {
        self.s.put(l as usize)
    }
    
    pub fn fmt_debug<T: fmt::Debug>(&self, g: &Game<T, 1>, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter()
            .map(|l| g.data(l))
            .format_with("|", |x, f|
                f(&format_args!("{:?}", x))
            )
        )
    }

    pub fn fmt_display<T: fmt::Display>(&self, g: &Game<T, 1>, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter()
            .map(|l| g.data(l))
            .format("|")
        )
    }
}

impl BitAndAssign<&Self> for LocSet {
    fn bitand_assign(&mut self, rhs: &Self) {
        assert_eq!(self.s.len(), rhs.s.len());
        self.s &= &rhs.s
    }
}

impl Extend<Loc> for LocSet {
    fn extend<T: IntoIterator<Item=Loc>>(&mut self, iter: T) {
        for l in iter {
            self.insert(l);
        }
    }
}
