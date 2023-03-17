use crate::*;

macro_rules! newtype {
    ($name:ident, $t:ty) => {

#[derive(new, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct $name(pub $t);

impl $name {
    pub fn index(self) -> usize {
        self.0 as usize
    }
    pub fn value(self) -> $t {
        self.0
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

impl From<$name> for $t {
    fn from(t: $name) -> $t {
        t.0
    }
}

impl TryFrom<usize> for $name {
    type Error = <$t as TryFrom<usize>>::Error;

    fn try_from(t: usize) -> Result<Self, Self::Error> {
        <$t as TryFrom<usize>>::try_from(t)
            .map(|x| $name(x))
    }
}

    };
}


newtype!(Loc, u32);
newtype!(Obs, u32);
newtype!(Act, u16);
newtype!(Agt, u8);

newtype!(TransducerState, u32);

pub fn loc(l: impl TryInto<u32>) -> Loc {
    Loc::new(l.try_into().ok().unwrap())
}

pub fn obs(o: impl TryInto<u32>) -> Obs {
    Obs::new(o.try_into().ok().unwrap())
}

pub fn act(o: impl TryInto<u16>) -> Act {
    Act::new(o.try_into().ok().unwrap())
}

pub fn agt(o: impl TryInto<u8>) -> Agt {
    Agt::new(o.try_into().ok().unwrap())
}

pub fn transducer_state(s: impl TryInto<u32>) -> TransducerState {
    TransducerState::new(s.try_into().ok().unwrap())
}
