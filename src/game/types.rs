use num::ToPrimitive;
use crate::*;

macro_rules! newtype {
    ($name:ident, $t:ty) => {
#[derive(new)]
pub struct $name<T>($t, PhantomData<T>);

impl<T> Ord for $name<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> PartialOrd for $name<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> $name<T> {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl<T> Clone for $name<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}
impl<T> Copy for $name<T> {}
impl<T> PartialEq for $name<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for $name<T> {}
impl<T> Hash for $name<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Debug for $name<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T> From<$name<T>> for usize {
    fn from(t: $name<T>) -> usize {
        t.index()
    }
}
    };
}


newtype!(Loc, u32);
newtype!(Obs, u32);
newtype!(TransducerState, u32);

pub fn loc<T>(l: impl ToPrimitive) -> Loc<T> {
    Loc::new(l.to_u32().unwrap())
}

pub fn obs<T>(o: impl ToPrimitive) -> Obs<T> {
    Obs::new(o.to_u32().unwrap())
}

pub fn transducer_state<T>(s: impl ToPrimitive) -> TransducerState<T> {
    TransducerState::new(s.to_u32().unwrap())
}
