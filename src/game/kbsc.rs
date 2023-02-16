use crate::*;

#[derive(new, Debug, Clone)]
pub struct KBSC {
    pub g: Rc<Game<1>>
}

impl AbstractGame<1> for KBSC {
    type Loc = Interned<LocSet>;
    type Obs = Interned<LocSet>;

    fn l0(&self) -> Self::Loc { LocSet::singleton(&self.g, self.g.l0()).intern() }
    fn n_actions(&self) -> [usize; 1] { self.g.n_actions }
    fn obs(&self, s: &Self::Loc) -> [Self::Obs; 1] { [s.clone()] }
    fn is_winning(&self, s: &Self::Loc) -> bool {
        s.iter().all(|l| self.g.is_winning(l))
    }

    fn succ(&self, s: &Self::Loc, mut f: impl FnMut([Act; 1], Self::Loc)) {
        let g = &self.g;

        let mut succ = BTreeSet::new();
        
        for l in s.iter() {
            for &([a], l2) in g.successors(l) {
                let [obs] = g.observe(l2);
                succ.insert((a, obs, l2));
            }
        }

        let mut subset = LocSet::new(&g);
        for ((a, obs), group) in &succ.iter().group_by(|(a, obs, _)| (*a, *obs)) {
            subset.clear();
            
            for (a_, obs_, l) in group {
                assert_eq!((a, obs), (*a_, *obs_));
                subset.put(*l);
            }

            f([a], subset.intern());
        }
    }

    fn fmt_loc(&self, f: &mut fmt::Formatter, s: &Self::Loc) -> fmt::Result {
        format_sep(f, "|", s.iter(), |f, l|
            self.g.fmt_loc(f, l)
        )
    }

    fn fmt_obs(&self, f: &mut fmt::Formatter, agt: Agt, o: &Self::Obs) -> fmt::Result {
        assert_eq!(agt, types::agt(0));
        self.fmt_loc(f, o)
    }

    fn fmt_act(&self, f: &mut fmt::Formatter, a: Act) -> fmt::Result {
        self.g.fmt_act(f, a)
    }
}
