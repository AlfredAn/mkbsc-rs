use crate::*;

#[derive(Debug)]
pub struct ObsSubset<T> {
    pub obs: Obs<T>,
    pub set: FixedBitSet
}

impl<T> Hash for ObsSubset<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.obs.hash(state);
        self.set.hash(state);
    }
}

impl<T> Eq for ObsSubset<T> {}

impl<T> PartialEq for ObsSubset<T> {
    fn eq(&self, other: &Self) -> bool {
        self.obs == other.obs && self.set == other.set
    }
}

impl<T> Clone for ObsSubset<T> {
    fn clone(&self) -> Self {
        Self { obs: self.obs.clone(), set: self.set.clone() }
    }
}

impl<T> ObsSubset<T> {
    pub fn new(g: &Game<T, 1>, obs: Obs<T>) -> Self {
        Self {
            obs,
            set: FixedBitSet::with_capacity(g.obs_set(0, obs).len())
        }
    }

    pub fn s0(g: &Game<T, 1>) -> Self {
        let ([obs], [off]) = (g.observe(loc(0)), g.obs_offset(loc(0)));
        let mut result = Self::new(g, obs);
        result.set.put(off as usize);
        result
    }

    pub fn iter<'a>(&'a self, g: &'a Game<T, 1>) -> impl Iterator<Item=Loc<T>> + 'a {
        self.set.ones()
            .map(|i| self.loc(g, i))
    }

    pub fn put(&mut self, g: &Game<T, 1>, l: Loc<T>) -> bool {
        assert_eq!(g.observe(l), [self.obs]);
        self.set.put(g.obs_offset(l)[0] as usize)
    }

    pub fn is_empty(&self) -> bool {
        self.set.count_ones(..) == 0
    }

    fn loc(&self, g: &Game<T, 1>, i: usize) -> Loc<T> {
        g.obs_set(0, self.obs)[i]
    }

    pub fn fmt_debug(&self, g: &Game<T, 1>, f: &mut fmt::Formatter) -> fmt::Result
    where T: Debug {
        write!(f, "{}", self.iter(g)
            .map(|l| g.data(l))
            .format_with("|", |x, f|
                f(&format_args!("{:?}", x))
            )
        )
    }

    pub fn fmt_display(&self, g: &Game<T, 1>, f: &mut fmt::Formatter) -> fmt::Result
    where T: Display {
        write!(f, "{}", self.iter(g)
            .map(|l| g.data(l))
            .format("|")
        )
    }
}
