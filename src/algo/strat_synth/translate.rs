use crate::*;

pub fn translate_strategy<'a, const N: usize>(
    stack: &'a MKBSCStack<N>,
    agt: Agt,
    strat: impl MemorylessStrategy + 'a
) -> impl Strategy + 'a {
    MKBSCStrategyTranslation::new(stack, agt, strat)
}

pub fn proj_to_base_strategy<const N: usize>(
    proj: ConstructedGame<Project<N>, 1>,
    strat: impl Strategy
) -> impl Strategy {
    strategy_alt(
        strat.init_tuple(),
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
                strategy_alt(
                    strat.init_tuple(),
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
                strategy_alt(
                    strat.init_tuple(),
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
struct MKBSCStrategyTranslation<'a, S: MemorylessStrategy, const N: usize> {
    stack: &'a MKBSCStack<N>,
    agt: Agt,
    strat: S // strategy for (G^(jK)|_i)^K, where j=stack.len()-1
}

// impl strategy for G
impl<'a, S: MemorylessStrategy, const N: usize> Strategy for MKBSCStrategyTranslation<'a, S, N> {
    type M = Loc; // Loc in (G^(jK)|_i)^K

    fn update(&self, o_g: Obs, &s: &Self::M) -> Option<Self::M> {
        let j = self.j();
        let agt = self.agt;

        let g = &self.stack.base;
        let gk = self.gk();

        let s2 = gk.post(s, [self.action(&s)])
            .filter(|&s2| {
                let mut s_k = s2;
                for k in j..=0 {
                    let gi_k = self.stack.projection(k, agt);
                    let gk_k = self.stack.kbsc(k, agt);

                    let l_gi = gk_k.origin_loc(s_k).first().unwrap();
                    let &l_g = gi_k.origin_loc(l_gi);

                    s_k = match self.stack.get(k) {
                        StackElement::MKBSC(g_k) => g_k.origin_loc(l_g)[agt.index()],
                        StackElement::Base(_) => l_g
                    };
                }
                let l = s_k;
                g.observe(l)[agt.index()] == o_g
            })
            .at_most_one()
            .ok().unwrap();
        
        if let Some(s2) = s2 {
            if self.strat.call_ml(gk.observe(s2)[0]).is_some() {
                Some(s2)
            } else { None }
        } else { None }
    }

    fn action(&self, &s: &Self::M) -> Act {
        self.strat.call_ml(
            self.gk().observe(s)[0]
        ).unwrap()
    }

    fn init(&self) -> Self::M {
        self.gk().l0()
    }
}

impl<'a, S: MemorylessStrategy, const N: usize> MKBSCStrategyTranslation<'a, S, N> {
    fn j(&self) -> usize {
        self.stack.len().checked_sub(2).unwrap()
    }

    fn gk(&self) -> &Rc<Game<1>> {
        &self.stack.kbsc(self.j(), self.agt).game
    }
}
