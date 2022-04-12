use super::*;

pub struct ObsSubset<T> {
    g: Rc<Game<T, 1>>,
    obs: game::Obs,
    set: FixedBitSet
}

impl<T> ObsSubset<T> {
    pub fn new(g: Rc<Game<T, 1>>, obs: Obs) -> Self {
        Self {
            obs,
            set: FixedBitSet::with_capacity(g.obs_set(0, obs).len()),
            g
        }
    }

    pub fn s0(g: Rc<Game<T, 1>>) -> Self {
        let ([obs], [off]) = (g.observe(0), g.obs_offset(0));
        let mut result = Self::new(g, obs);
        result.set.put(off as usize);
        result
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=Loc> + 'a {
        self.set.ones()
            .map(|i| self.g.obs_set(0, self.obs)[i])
    }

    pub fn put(&mut self, l: Loc) -> bool {
        assert_eq!(self.g.observe(l), [self.obs]);
        self.set.put(self.g.obs_offset(l)[0] as usize)
    }
}

impl<T> Clone for ObsSubset<T> {
    fn clone(&self) -> Self {
        Self {
            g: self.g.clone(),
            obs: self.obs,
            set: self.set.clone()
        }
    }
}

impl<T> PartialEq for ObsSubset<T> {
    fn eq(&self, rhs: &Self) -> bool {
        (self.obs, &self.set) == (rhs.obs, &rhs.set)
    }
}

impl<T> Eq for ObsSubset<T> {}

impl<T> Hash for ObsSubset<T> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.obs.hash(h);
        self.set.hash(h);
    }
}

impl<T: fmt::Debug> fmt::Debug for ObsSubset<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", self.iter()
            .map(|l| self.g.data(l))
            .format_with("|", |x, f|
                f(&format_args!("{:?}", x))
            )
        )
    }
}

impl<T: fmt::Display> fmt::Display for ObsSubset<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter()
            .map(|l| self.g.data(l))
            .format("|")
        )
    }
}
