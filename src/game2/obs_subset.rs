use crate::util::fbs_filter;
//use LocSet::*;
use super::*;

/*#[derive(From, TryInto, IsVariant, Unwrap, Debug, PartialEq, Eq, Hash, SmartDefault)]
pub enum LocSet<T> {
    #[default]
    Empty,
    #[from]
    ObsSubset(ObsSubset<T>)
}

impl<T> Clone for LocSet<T> {
    fn clone(&self) -> Self {
        match self {
            Empty => Empty,
            ObsSubset(s) => ObsSubset(s.clone())
        }
    }
}

impl<T> LocSet<T> {
    pub fn iter(&self) -> impl Iterator<Item=Loc> + '_ {
        match self {
            Empty => None,
            ObsSubset(s) => Some(s.iter())
        }.into_iter().flatten()
    }

    pub fn put(&mut self, l: Loc) -> bool {
        match self {
            Empty => panic!(),
            ObsSubset(s) => s.put(l)
        }
    }
}*/

/*impl<T> BitAndAssign<&Self> for LocSet<T> {
    fn bitand_assign(&mut self, rhs: &Self) {
        *self = match (mem::take(self), rhs) {
            (Empty, _) | (_, Empty) => Empty,
            (ObsSubset(mut s), ObsSubset(s2)) => {
                if Rc::ptr_eq(&s.g, &s2.g) {
                    if s.obs == s2.obs {
                        if fbs_intersection_check_empty(&mut s.set, &s2.set) {
                            Empty
                        } else {
                            ObsSubset(s)
                        }
                    } else {
                        Empty
                    }
                } else {
                    // assumes the subsets are sorted
                    let mut itr = s2.iter();
                    let mut next = itr.next();
                    let count = s.filter(|l| {
                        while let Some(l2) = next {
                            if l2 < l {
                                next = itr.next();
                            } else if l2 == l {
                                next = itr.next();
                                return true;
                            } else { // l2 > l
                                return false;
                            }
                        }
                        false
                    });
                    if count > 0 {
                        ObsSubset(s)
                    } else {
                        Empty
                    }
                }
            }
        };
    }
}*/

/*impl BitAndAssign<&Self> for ObsSubset {
    fn bitand_assign(&mut self, rhs: &Self) {
        if Rc::ptr_eq(&self.g, &rhs.g) {
            if self.obs == rhs.obs {
                self.set &= &rhs.set;
            } else {
                self.set.clear();
            }
        } else {
            // assumes the subsets are sorted
            let mut itr = rhs.iter();
            let mut next = itr.next();
            self.filter(|l| {
                while let Some(l2) = next {
                    if l2 < l {
                        next = itr.next();
                    } else if l2 == l {
                        next = itr.next();
                        return true;
                    } else { // l2 > l
                        return false;
                    }
                }
                false
            });
        }
    }
}*/

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

    pub fn filter<T>(&mut self, g: &Game<T, 1>, mut f: impl FnMut(Loc) -> bool) -> usize {
        fbs_filter(&mut self.set, |i| f(g.obs_set(0, self.obs)[i]))
    }

    pub fn put<T>(&mut self, g: &Game<T, 1>, l: Loc) -> bool {
        assert_eq!(g.observe(l), [self.obs]);
        self.set.put(g.obs_offset(l)[0] as usize)
    }

    /*pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }*/

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
