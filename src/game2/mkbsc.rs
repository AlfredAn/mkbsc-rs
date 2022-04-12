use super::*;

#[derive(Debug, Clone)]
pub struct MKBSC<T, const N: usize> {
    pub g: Rc<Game<T, N>>,
    pub gi: [Rc<Game<T, 1>>; N],
    pub gki: [Rc<Game<ObsSubset<T>, 1>>; N]
}

impl<T: Clone, const N: usize> MKBSC<T, N> {
    pub fn new(g: Rc<Game<T, N>>) -> Self {
        let gi = array_init(|i|
            Rc::new(Project::new(g.clone(), i).build())
        );
        let gki = array_init(|i|
            Rc::new(KBSC::new(gi[i].clone()).build())
        );

        Self {
            g, gi, gki
        }
    }
}

impl<T: Clone, const N: usize> AbstractGame<N> for MKBSC<T, N> {
    type Loc = [game::Loc; N];
    type Obs = game::Loc;
    type Data = [ObsSubset<T>; N];

    fn l0(&self) -> Self::Loc { [0; N] }
    fn n_actions(&self) -> [usize; N] { self.g.n_actions }
    fn obs(&self, &s: &Self::Loc) -> [Self::Obs; N] { s }

    fn is_winning(&self, &s: &Self::Loc) -> bool {
        (0..N).any(|i|
            self.gki[i].is_winning(s[i])
        )
    }

    fn data(&self, &s: &Self::Loc) -> Self::Data {
        array_init(|i|
            self.gki[i].data(s[i]).clone()
        )
    }

    fn succ(&self, &s: &Self::Loc, mut f: impl FnMut([Act; N], Self::Loc)) {
        let succ_gki: [_; N] = array_init(|i|
            self.gki[i].succ(s[i])
        );

        cartesian_product(succ_gki, |x| {
            let a = x.map(|&([a], _)| a);
            let s2 = x.map(|&(_, si2)| si2);
            

            f(a, s2);
        });
    }
}
