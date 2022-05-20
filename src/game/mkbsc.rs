use crate::*;

#[derive(Debug, Clone)]
pub struct MKBSC<const N: usize> {
    pub g: Rc<Game<N>>,
    pub gi: [ConstructedGame<Project<N>, 1>; N],
    pub gki: [ConstructedGame<KBSC, 1>; N],
    buf: RefCell<[LocSet; 2]>
}

impl<const N: usize> MKBSC<N> {
    pub fn new(g: Rc<Game<N>>) -> Self {
        let gi: [_; N] = array_init(|i|
            Project::new(g.clone(), agt(i)).build()
        );
        let gki: [_; N] = array_init(|i|
            KBSC::new(gi[i].game.clone()).build()
        );

        Self {
            buf: RefCell::new(array_init(|_| LocSet::new(&*g))),
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
        let mut buf = self.buf.borrow_mut();
        let [pre_locs, post_g] = buf.ref_array_mut();

        pre_locs.replace_with(self.gki[0].origin_loc(s[0]));
        for i in 1..N {
            *pre_locs &= self.gki[i].origin_loc(s[i]);
        }

        let mut slices: [_; N] = array_init(|_| vec![]);

        for a in self.g.action_profiles() {
            post_g.clear();

            let mut itr = self.g.post_set(pre_locs.iter(), a);
            if let Some(first) = itr.next() {
                post_g.insert(first);
            } else {
                continue;
            }

            post_g.extend(itr);

            for i in 0..N {
                slices[i].clear();
                let post_gki = self.gki[i].post_raw(s[i], [a[i]]);

                for &(_, si2) in post_gki {
                    let si = &**self.gki[i].origin_loc(si2);

                    // TODO: remove slow workaround
                    let si = LocSet::from_iter(
                        &self.g,
                        si.iter().map(|l_gi|
                            *self.gi[i].origin_loc(l_gi)
                        )
                    );

                    slices[i].push((si2, si));
                }
            }

            let slices: [_; N] = from_iter(
                slices.iter().map(|v| &**v)
            ).unwrap();

            cartesian_product(slices, |x| {
                for (i, &(mut block)) in post_g.raw().iter().enumerate() {
                    for (_, set) in x {
                        block &= set.raw()[i]
                    }
                    if block != 0 {
                        let s2 = x.map(|&(si2, _)| si2);
                        f(a, s2);
                        break;
                    }
                }
            });
        }
    }

    fn fmt_loc(&self, f: &mut fmt::Formatter, s: &Self::Loc) -> fmt::Result {
        format_sequence(f, SequenceFormat {sep: ",", ..LIST}, 0..N, |f, i|
            self.gki[i].fmt_loc(f, s[i])
        )
    }
}

impl<'a, const N: usize> ConstructedGame<MKBSC<N>, N> {
    pub fn to_kbsc_profile(
        &self,
        strat_gk: [impl Strategy + 'a; N]
    ) -> [impl Strategy + 'a; N] {
        mkbsc_to_kbsc_profile(self.clone(), strat_gk)
    }

    pub fn from_kbsc_profile(
        &self,
        strat_gki: [impl Strategy + 'a; N]
    ) -> [impl Strategy + 'a; N] {
        kbsc_to_mkbsc_profile(self.clone(), strat_gki)
    }
}
