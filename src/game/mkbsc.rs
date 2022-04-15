use crate::*;

#[derive(Debug, Clone)]
pub struct MKBSC<T, const N: usize> {
    pub g: Rc<Game<T, N>>,
    pub gi: [Rc<Game<T, 1>>; N],
    pub gki: [Rc<Game<KBSCData<T>, 1>>; N]
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

    pub fn data_ref(&self, s: [Loc; N]) -> [&KBSCData<T>; N] {
        array_init(|i|
            self.gki[i].data(s[i])
        )
    }
}

impl<T: Clone, const N: usize> AbstractGame<N> for MKBSC<T, N> {
    type Loc = [Loc; N];
    type Obs = Loc;
    type Data = [KBSCData<T>; N];

    fn l0(&self) -> Self::Loc { [0; N] }
    fn n_actions(&self) -> [usize; N] { self.g.n_actions }
    fn obs(&self, &s: &Self::Loc) -> [Self::Obs; N] { s }

    fn is_winning(&self, &s: &Self::Loc) -> bool {
        (0..N).any(|i|
            self.gki[i].is_winning(s[i])
        )
    }

    fn data(&self, &s: &Self::Loc) -> Self::Data {
        self.data_ref(s).map(|si| si.clone())
    }

    fn succ(&self, &s: &Self::Loc, mut f: impl FnMut([Act; N], Self::Loc)) {
        let t = self.data_ref(s);
        let mut pre_locs = t[0].s.clone();
        for &ti in &t[1..] {
            pre_locs &= &ti.s;
        }
        assert!(!pre_locs.is_empty());

        //println!("\n\nt: {:?}", t);
        for a in self.g.action_profiles() {
            let post_g = LocSet::from_iter(
                &self.g,
                self.g.post_set(pre_locs.iter(), a)
            );

            if post_g.is_empty() { continue; }

            let post_gki: [_; N] = array_init(|i|
                self.gki[i].post_raw(s[i], [a[i]])
            );
            //println!("  post_gki: {:?}", post_gki);
    
            cartesian_product(post_gki, |x| {
                let s2 = x.map(|&(_, si2)| si2);
                let t2 = self.data_ref(s2);
                //println!("    t2: {:?}", t2);

                let mut possible = post_g.clone();
                for ti in t2 {
                    possible &= &ti.s;
                }

                if !possible.is_empty() {
                    f(a, s2);
                }
            });
        }
    }
}
