use crate::game2::AbstractGame;
use crate::game2::Act;
use enumset::*;
use self::Loc::*;

#[derive(Debug, Clone, Copy)]
pub struct CupGame();

#[derive(EnumSetType, Debug, Hash)]
pub enum Loc {
    Start, Bad, Good, Lose, Win
}

impl AbstractGame<2> for CupGame {
    type Loc = Loc;
    type Obs = EnumSet<Loc>;

    fn l0(&self) -> Self::Loc { Start }
    fn n_actions(&self) -> [usize; 2] { [3, 3] }
    fn obs(&self, loc: &Self::Loc) -> [Self::Obs; 2] {[
        enum_set!(loc),
        match loc {
            Bad | Good => enum_set!(Bad | Good),
            l => enum_set!(l)
        }
    ]}
    fn is_winning(&self, &loc: &Self::Loc) -> bool { loc == Win }

    fn succ(
        &self,
        loc: &Self::Loc,
        mut f: impl FnMut([Act; 2], Self::Loc)
    ) {
        let (g, l, s) = (0, 1, 2);

        macro_rules! e {
            [$($x:expr),*] => {
                for (a0, a1, l) in [$($x),*] {
                    f([a0, a1], l);
                }
            };
        }

        match loc {
            Start => e![(g, g, Bad), (g, g, Good)],
            Bad => e![(l, l, Lose), (l, s, Lose), (s, l, Lose), (s, s, Good)],
            Good => e![(l, l, Win), (l, s, Lose), (s, l, Lose), (s, s, Good)],
            Lose => e![(s, s, Lose)],
            Win => e![(l, l, Win)],
        };
    }
}
