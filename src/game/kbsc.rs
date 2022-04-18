use crate::*;

#[derive(new, Debug, Clone)]
pub struct KBSC<T> {
    pub g: Rc<Game<T, 1>>
}

#[derive(new, Clone)]
pub struct KBSCData<T> {
    pub g: Rc<Game<T, 1>>,
    pub obs: Obs<T>,
    pub s: LocSet<T>
}

/*thread_local!(
    static TEMP: RefCell<BTreeSet<(Act, Obs, Loc<T>)>> = Default::default();
);*/

impl<T> AbstractGame<1> for KBSC<T> {
    type Loc = ObsSubset<T>;
    type Obs = ObsSubset<T>;
    type Data = KBSCData<T>;

    fn l0(&self) -> Self::Loc { ObsSubset::s0(&self.g) }
    fn n_actions(&self) -> [usize; 1] { self.g.n_actions }
    fn obs(&self, s: &Self::Loc) -> [Self::Obs; 1] { [s.clone()] }
    fn is_winning(&self, s: &Self::Loc) -> bool {
        s.iter(&self.g).all(|l| self.g.is_winning(l))
    }
    fn data(&self, s: &Self::Loc) -> Self::Data {
        KBSCData::new(self.g.clone(), s.obs, LocSet::from_subset(&self.g, s))
    }

    fn succ(&self, s: &Self::Loc, mut f: impl FnMut([Act; 1], Self::Loc)) {
        let g = &self.g;

        //println!("s={:?}", s);

        let mut succ = BTreeSet::new();
        
        //TEMP.with(|r| {
            //let mut succ = r.borrow_mut();
            //succ.clear();

            for l in s.iter(g) {
                for &([a], l2) in g.successors(l) {
                    let [obs] = g.observe(l2);
                    succ.insert((a, obs, l2));
                }
            }

            for ((a, obs), group) in &succ.iter().group_by(|(a, obs, _)| (*a, *obs)) {
                let mut subset = ObsSubset::new(&g, obs);
                //println!("  group(a={:?}, o={:?})", a, obs);
                
                for (a_, obs_, l) in group {
                    assert_eq!((a, obs), (*a_, *obs_));
                    //println!("    l={:?}", *l);
                    subset.put(&g, *l);
                }
                f([a], subset);
            }
        //});
    }
}

impl<T: fmt::Debug> fmt::Debug for KBSCData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.s.fmt_debug(&self.g, f)
    }
}

impl<T: fmt::Display> fmt::Display for KBSCData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.s.fmt_display(&self.g, f)
    }
}
