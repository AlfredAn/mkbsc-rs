use crate::*;

macro_rules! newtype {
    ($name:ident, $t:ty) => {

#[derive(new, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct $name($t);

impl $name {
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl Debug for $name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Display for $name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<$name> for usize {
    fn from(t: $name) -> usize {
        t.index()
    }
}
    };
}


newtype!(Loc, u32);
newtype!(Obs, u32);
newtype!(TransducerState, u32);

pub fn loc(l: impl TryInto<u32>) -> Loc {
    Loc::new(l.try_into().ok().unwrap())
}

pub fn obs(o: impl TryInto<u32>) -> Obs {
    Obs::new(o.try_into().ok().unwrap())
}

pub fn transducer_state(s: impl TryInto<u32>) -> TransducerState {
    TransducerState::new(s.try_into().ok().unwrap())
}
