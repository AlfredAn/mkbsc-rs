/*use anyhow::bail;

use crate::*;

pub struct CoinGame;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Loc {
    Start,
    Win,
    Play([bool; 3])
}

fn count(x: &[bool]) -> u8 {
    x.iter()
        .fold(0, |c, &v| c + v as u8)
}

impl Loc {
    pub fn count(self) -> Option<u8> {
        match self {
            Loc::Play(x) => Some(count(&x)),
            _ => None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Obs {
    Start,
    Win,
    Zero,
    One,
    Two
}

impl From<[bool; 2]> for Obs {
    fn from(x: [bool; 2]) -> Self {
        count(&x)
            .try_into()
            .unwrap()
    }
}

impl TryFrom<u8> for Obs {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Obs::Zero,
            1 => Obs::One,
            2 => Obs::Two,
            _ => bail!("invalid value")
        })
    }
}

const NO_ACTION: Act = Act(3);

impl AbstractGame<2> for CoinGame {
    type Loc = Loc;
    type Obs = Obs;

    fn l0(&self) -> Loc {
        Loc::Start
    }

    fn n_actions(&self) -> [usize; 2] {
        [4; 2]
    }

    fn obs(&self, &l: &Loc) -> [Obs; 2] {
        match l {
            Loc::Start => [Obs::Start; 2],
            Loc::Win => [Obs::Win; 2],
            Loc::Play([a, b, c]) => [[a, b].into(), [b, c].into()],
        }
    }

    fn is_winning(&self, &l: &Loc) -> bool {
        l == Loc::Win
    }

    fn succ(
        &self,
        l: &Loc,
        mut f: impl FnMut([Act; 2], Loc)
    ) {
        match l {
            Loc::Start => for x in [
                [true, true, false],
                [true, false, true],
                [false, true, true],
            ] {
                f([act(0); 2], Loc::Play(x))
            },
            Loc::Win => f([act(0); 2], Loc::Win),
            Loc::Play(x) => {
                cartesian_product_ints([4; 2], |a| {
                    let a = a.map(|a| act(a));

                    
                });
            },
        } 
    }

    fn fmt_loc(&self, f: &mut fmt::Formatter, l: &Loc) -> fmt::Result {
        write!(f, "{:?}", l)
    }
}
*/