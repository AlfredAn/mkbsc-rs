use itertools::Itertools;
use crate::into_dgame::into_dgame;
use std::rc::Rc;
use crate::{game::*, algo::*, dgame::*};

type DG<T> = DGame<T, 1>;
type GK<T> = KBSC<DG<T>, Rc<DG<T>>>;
type KLoc<T> = <GK<T> as Game<1>>::Loc;
type DGK<T> = DGame<KLoc<T>, 1>;

pub struct DKBSC<T: Clone> {
    dg: Rc<DG<T>>,
    gk: Rc<GK<T>>,
    dgk: Rc<DGK<T>>
}

impl<T: Clone> DKBSC<T> {
    pub fn new(dg: Rc<DG<T>>) -> Self {
        let gk = Rc::new(dg.clone().kbsc());

        let rk = into_dgame(&*gk);
        let dgk = Rc::new(rk.dg);

        Self {
            dg, gk, dgk
        }
    }
    
    pub fn translate_strategy<'b>(
        &'b self,
        strat: impl MemorylessStrategy1<DGK<T>> + 'b
    ) -> impl Strategy1<DG<T>> + 'b
    {
        strategy1(
            move |&obs: &ObsIndex, &si: &Option<NodeIndex>| {
                let dg = &self.dg;
                let dgk = &self.dgk;

                // map location in gk to observation in g
                let map_to_obs = |si| {
                    let s = &dgk.node(si).data;
                    dg.observe(s.iter().next().unwrap())[0] // all loc in s must have the same obs
                };

                let si: NodeIndex = if let Some(si) = si {
                    si
                } else {
                    assert_eq!(obs, map_to_obs(dgk.l0));
                    dgk.l0
                };
                
                if let Some(a) = strat.call_ml1(&dgk.observe1(&si)) {
                    let mut post = dgk.post1(&si, a)
                        .filter(|&si| map_to_obs(si) == obs)
                        .dedup();
                    let s2 = post.next();
                    assert!(post.next().is_none()); // should be guaranteed by theory

                    s2.map(|s2| (a, s2))
                } else {
                    None
                }
            }
        )
    }
}
