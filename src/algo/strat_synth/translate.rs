use crate::*;

pub fn translate_kbsc_strategy(
    gk: ConstructedGame<KBSC, 1>,
    strat: impl Strategy
) -> impl Strategy {
    KBSCStrategyTranslation::new(gk, strat)
}

pub fn proj_to_base_strategy<const N: usize>(
    proj: ConstructedGame<Project<N>, 1>,
    strat: impl Strategy
) -> impl Strategy {
    strategy(
        strat.init(),
        move |o_g, mem| {
            strat.call(proj.obs(&(agt(0), o_g)), mem)
        }
    )
}

pub fn kbsc_to_mkbsc_profile<'a, const N: usize>(
    gk: ConstructedGame<MKBSC<N>, N>,
    strat: [impl Strategy + 'a; N]
) -> [impl Strategy + 'a; N] {
    from_iter(
        strat.into_iter()
            .enumerate()
            .map(|(i, strat)| {
                let gk = gk.clone();
                strategy(
                    strat.init(),
                    move |o_gk, mem| {
                        let gki = &gk.origin().gki[i];

                        let &l_gki = gk.origin_obs(agt(i), o_gk);
                        let [o_gki] = gki.observe(l_gki);
                        
                        strat.call(o_gki, mem)
                    }
                )
            })
    ).unwrap()
}

pub fn mkbsc_to_kbsc_profile<'a, const N: usize>(
    gk: ConstructedGame<MKBSC<N>, N>,
    strat: [impl Strategy + 'a; N]
) -> [impl Strategy + 'a; N] {
    from_iter(
        strat.into_iter()
            .enumerate()
            .map(|(i, strat)| {
                let gk = gk.clone();
                strategy(
                    strat.init(),
                    move |o_gki, mem| {
                        let gki = &gk.origin().gki[i];

                        let l_gki = gki.to_unique_loc(o_gki, agt(0)).unwrap();
                        let o_gk = gk.obs(&(agt(i), l_gki));

                        strat.call(o_gk, mem)
                    }
                )
            })
    ).unwrap()
}

#[derive(new, Debug, Clone)]
struct KBSCStrategyTranslation<S: Strategy> {
    gk: ConstructedGame<KBSC, 1>,
    strat: S // strategy for G^K
}

// impl strategy for G
impl<S: Strategy> Strategy for KBSCStrategyTranslation<S> {
    type M = (Option<(Loc, Act)>, S::M); // Loc in G^K

    fn call(&self, o_g: Obs, (la0, m): &Self::M) -> Option<(Act, Self::M)> {
        let g = &self.gk.origin().g;
        let gk = &self.gk;
        let strat = &self.strat;

        // map location in gk to observation in g
        let map_to_obs = |l_gk| {
            let l_g = self.gk.origin_loc(l_gk).first().unwrap();
            let [o_g] = g.observe(l_g);
            o_g
        };

        let l_gk = if let &Some((l0_gk, a0)) = la0 {
            let mut post = gk.post(l0_gk, [a0])
                .filter(|&l_gk| map_to_obs(l_gk) == o_g);
            
            if let Some(l_gk) = post.next() {
                assert!(post.next().is_none()); // should be guaranteed by theory
                l_gk
            } else {
                return None;
            }
        } else {
            gk.l0()
        };

        let [o_gk] = gk.observe(l_gk);

        if let Some((a, m2)) = strat.call(o_gk, &m) {
            return Some((a, (Some((l_gk, a)), m2)));
        }
        
        None
    }

    fn init(&self) -> Self::M {
        (None, self.strat.init())
    }
}
