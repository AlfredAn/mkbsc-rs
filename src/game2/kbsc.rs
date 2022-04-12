use super::*;

#[derive(new, Debug, Clone)]
pub struct KBSC<T> {
    g: Rc<Game<T, 1>>
}

thread_local!(
    static TEMP: RefCell<BTreeSet<(Act, Obs, Loc)>> = Default::default();
);

impl<T> AbstractGame<1> for KBSC<T> {
    type Loc = ObsSubset<T>;
    type Obs = Self::Loc;
    type Data = ObsSubset<T>;

    fn l0(&self) -> Self::Loc { ObsSubset::s0(self.g.clone()) }
    fn n_actions(&self) -> [usize; 1] { self.g.n_actions }
    fn obs(&self, s: &Self::Loc) -> [Self::Obs; 1] { [s.clone()] }
    fn is_winning(&self, s: &Self::Loc) -> bool {
        s.iter().all(|l| self.g.is_winning(l))
    }
    fn data(&self, s: &Self::Loc) -> Self::Data {
        s.clone()
    }

    fn succ(&self, s: &Self::Loc, mut f: impl FnMut([Act; 1], Self::Loc)) {
        let g = &self.g;
        
        TEMP.with(|r| {
            let mut succ = r.borrow_mut();
            succ.clear();

            for l in s.iter() {
                for &([a], l2) in g.succ(l) {
                    let [obs] = g.observe(l2);
                    succ.insert((a, obs, l2));
                }
            }

            for ((a, obs), group) in &succ.iter().group_by(|(a, obs, _)| (*a, *obs)) {
                let mut subset = ObsSubset::new(g.clone(), obs);
                
                for (_, _, l) in group {
                    subset.put(*l);
                }
                f([a], subset);
            }
        });
    }
}
