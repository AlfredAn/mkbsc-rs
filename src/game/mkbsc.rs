use crate::*;

#[derive(Debug, Clone)]
pub struct MKBSC<const N: usize> {
    pub g: Rc<Game<N>>,
    pub gi: [ConstructedGame<Project<N>, 1>; N],
    pub gki: [ConstructedGame<KBSC, 1>; N]
}

impl<const N: usize> MKBSC<N> {
    pub fn new(g: Rc<Game<N>>) -> Self {
        let gi: [_; N] = array_init(|i|
            Project::new(g.clone(), i).build()
        );
        let gki: [_; N] = array_init(|i|
            KBSC::new(gi[i].game.clone()).build()
        );

        Self {
            g, gi, gki
        }
    }
}

impl<const N: usize> AbstractGame<N> for MKBSC<N> {
    type Loc = [Loc; N];
    type Obs = Loc;

    fn l0(&self) -> Self::Loc { array_init(|i| self.gki[i].l0()) }
    fn n_actions(&self) -> [usize; N] { self.g.n_actions }
    fn obs(&self, &s: &Self::Loc) -> [Self::Obs; N] { s }

    fn is_winning(&self, &s: &Self::Loc) -> bool {
        (0..N).any(|i|
            self.gki[i].is_winning(s[i])
        )
    }

    fn succ(&self, &s: &Self::Loc, mut f: impl FnMut([Act; N], Self::Loc)) {
        let mut pre_locs: LocSet = self.gki[0].origin_loc(s[0]).clone();
        for i in 1..N {
            pre_locs &= self.gki[i].origin_loc(s[i]);
        }
        assert!(!pre_locs.is_empty());

        for a in self.g.action_profiles() {
            let post_g = LocSet::from_iter(
                &self.g,
                self.g.post_set(pre_locs.iter(), a)
            );

            if post_g.is_empty() { continue; }

            let post_gki: [_; N] = array_init(|i|
                self.gki[i].post_raw(s[i], [a[i]])
            );
    
            cartesian_product(post_gki, |x| {
                let s2 = x.map(|&(_, si2)| si2);

                let mut possible = post_g.clone();
                for i in 0..N {
                    possible &= self.gki[i].origin_loc(s2[i]);
                }

                if !possible.is_empty() {
                    f(a, s2);
                }
            });
        }
    }

    fn fmt_loc(&self, f: &mut fmt::Formatter, s: &Self::Loc) -> fmt::Result {
        format_list(f, 0..N, |f, i|
            self.gki[i].fmt_loc(f, s[i])
        )
    }
}
