use crate::*;

#[derive(Debug)]
pub struct ObsSubset {
    pub obs: Obs,
    pub set: FixedBitSet
}

impl Hash for ObsSubset {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.obs.hash(state);
        self.set.hash(state);
    }
}

impl Eq for ObsSubset {}

impl PartialEq for ObsSubset {
    fn eq(&self, other: &Self) -> bool {
        self.obs == other.obs && self.set == other.set
    }
}

impl Clone for ObsSubset {
    fn clone(&self) -> Self {
        Self { obs: self.obs.clone(), set: self.set.clone() }
    }
}

impl ObsSubset {
    pub fn new(g: &Game<1>, obs: Obs) -> Self {
        Self {
            obs,
            set: FixedBitSet::with_capacity(g.obs_set(0, obs).len())
        }
    }

    pub fn s0(g: &Game<1>) -> Self {
        let ([obs], [off]) = (g.observe(loc(0)), g.obs_offset(loc(0)));
        let mut result = Self::new(g, obs);
        result.set.put(off as usize);
        result
    }

    pub fn iter<'a>(&'a self, g: &'a Game<1>) -> impl Iterator<Item=Loc> + 'a {
        self.set.ones()
            .map(|i| self.loc(g, i))
    }

    pub fn put(&mut self, g: &Game<1>, l: Loc) -> bool {
        assert_eq!(g.observe(l), [self.obs]);
        self.set.put(g.obs_offset(l)[0] as usize)
    }

    pub fn is_empty(&self) -> bool {
        self.set.count_ones(..) == 0
    }

    fn loc(&self, g: &Game<1>, i: usize) -> Loc {
        g.obs_set(0, self.obs)[i]
    }
}
