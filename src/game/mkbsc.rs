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

        // for (i, gi) in gi.iter().enumerate() {
        //     println!("p({}): {}", i, gi);
        // }
        // for (i, gki) in gki.iter().enumerate() {
        //     println!("k({}): {}", i, gki);
        // }

        Self {
            buf: RefCell::new([LocSet::new(&*g), LocSet::new(&*g)]),
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
        // println!("succ: {}", display(|f| self.fmt_loc(f, &s)));

        let mut buf = self.buf.borrow_mut();
        let buf = buf.split_at_mut(1);

        let pre_locs = &mut buf.0[0];
        // LocSet::intersect::<N>(pre_locs, array_init(|i| &**self.gki[i].origin_loc(s[i])));
        pre_locs.replace_with(self.gki[0].origin_loc(s[0]));
        for i in 1..N {
            *pre_locs &= self.gki[i].origin_loc(s[i]);
        }
        // assert!(!pre_locs.is_empty());

        // println!("  pre_locs: {}", display(|f| format_list(f, pre_locs.iter(), |f, l| self.g.fmt_loc(f, l))));

        let post_g = &mut buf.1[0];
        let mut slices: [_; N] = array_init(|_| vec![]);

        for a in self.g.action_profiles() {
            // println!("    a: {}", display(|f| format_list(f, a.iter(), |f, a| write!(f, "{}", a))));

            post_g.clear();

            let mut itr = self.g.post_set(pre_locs.iter(), a);
            if let Some(first) = itr.next() {
                post_g.insert(first);
            } else {
                continue;
            }

            post_g.extend(itr);

            // println!("      post_g: {}", display(|f| format_list(f, post_g.iter(), |f, l| self.g.fmt_loc(f, l))));

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
                // println!("        trying: {}", display(|f|
                //     format_list(f, x.iter(), |f, (_, s)|
                //         format_list(f, s.iter(), |f, l|
                //             self.g.fmt_loc(f, l)
                //         )
                //     )
                // ));

                for (i, &(mut block)) in post_g.raw().iter().enumerate() {
                    // println!("            {:#09b}", block);
                    for (_, set) in x {
                        // println!("            {:#09b}", set.raw()[i]);
                        block &= set.raw()[i]
                    }
                    if block != 0 {
                        // println!("          success");

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
    pub fn translate_strategy(
        &self,
        strat_gk: [impl Strategy + 'a; N]
    ) -> [impl Strategy + 'a; N] {
        let strat_gki = self.to_kbsc_profile(strat_gk);
        let strat_g = strat_gki.into_iter()
            .enumerate()
            .map(|(i, strat_gki)| {
                let strat_gi = self.origin().gki[i].translate_strategy(strat_gki);
                let strat_g = self.origin().gi[i].translate_strategy(strat_gi);
                strat_g
            });
        from_iter(strat_g).unwrap()
    }

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
