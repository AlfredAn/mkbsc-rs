use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObsSubset {
    obs: game::Obs,
    set: FixedBitSet
}

impl ObsSubset {
    pub fn new<T>(g: &Game<T, 1>, obs: Obs) -> Self {
        Self {
            obs,
            set: FixedBitSet::with_capacity(g.obs_set(0, obs).len())
        }
    }

    pub fn s0<T>(g: &Game<T, 1>) -> Self {
        let ([obs], [off]) = (g.observe(0), g.obs_offset(0));
        let mut result = Self::new(g, obs);
        result.set.put(off as usize);
        result
    }

    pub fn iter<'a, T>(&'a self, g: &'a Game<T, 1>) -> impl Iterator<Item=Loc> + 'a {
        self.set.ones()
            .map(|i| self.loc(g, i))
    }

    pub fn put<T>(&mut self, g: &Game<T, 1>, l: Loc) -> bool {
        assert_eq!(g.observe(l), [self.obs]);
        self.set.put(g.obs_offset(l)[0] as usize)
    }

    pub fn is_empty(&self) -> bool {
        self.set.count_ones(..) == 0
    }

    fn loc<T>(&self, g: &Game<T, 1>, i: usize) -> Loc {
        g.obs_set(0, self.obs)[i]
    }

    pub fn fmt_debug<T: fmt::Debug>(&self, g: &Game<T, 1>, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter(g)
            .map(|l| g.data(l))
            .format_with("|", |x, f|
                f(&format_args!("{:?}", x))
            )
        )
    }

    pub fn fmt_display<T: fmt::Display>(&self, g: &Game<T, 1>, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter(g)
            .map(|l| g.data(l))
            .format("|")
        )
    }
}
