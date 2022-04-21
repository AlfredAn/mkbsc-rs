use num::ToPrimitive;
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

pub fn loc(l: impl ToPrimitive) -> Loc {
    Loc::new(l.to_u32().unwrap())
}

pub fn obs(o: impl ToPrimitive) -> Obs {
    Obs::new(o.to_u32().unwrap())
}

pub fn transducer_state(s: impl ToPrimitive) -> TransducerState {
    TransducerState::new(s.to_u32().unwrap())
}
