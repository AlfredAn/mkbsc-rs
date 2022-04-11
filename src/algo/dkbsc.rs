use itertools::Itertools;
use crate::into_dgame::into_dgame;
use std::rc::Rc;
use crate::{game::*, algo::*, dgame::*};

type G<T> = DGame<T, 1>;
type GK<T> = KBSC<G<T>, Rc<G<T>>>;
type KLoc<T> = <GK<T> as Game<1>>::Loc;
type DGK<T> = DGame<KLoc<T>, 1>;

#[derive(Debug, Clone)]
pub struct DKBSC<T: Clone> {
    pub g: Rc<G<T>>,
    pub gk: Rc<DGK<T>>
}

impl<T: Clone> DKBSC<T> {
    pub fn new(g: Rc<G<T>>) -> Self {
        let gk0 = Rc::new(KBSC::<G<T>, _>::new(g.clone()));
        let gk = Rc::new(into_dgame(&*gk0).dg);

        Self {
            g, gk
        }
    }
    
    pub fn translate_strategy<'b>(
        &'b self,
        strat: impl MemorylessStrategy1<DGK<T>> + 'b
    ) -> impl Strategy1<G<T>> + 'b
    {
        strategy1(
            move |&obs: &ObsIndex, &si: &Option<NodeIndex>| {
                let g = &self.g;
                let gk = &self.gk;

                // map location in gk to observation in g
                let map_to_obs = |si| {
                    let s = &gk.node(si).data;
                    g.observe(s.iter().next().unwrap())[0] // all loc in s must have the same obs
                };

                let si: NodeIndex = if let Some(si) = si {
                    si
                } else {
                    assert_eq!(obs, map_to_obs(gk.l0));
                    gk.l0
                };
                
                if let Some(a) = strat.call_ml1(&gk.observe1(&si)) {
                    let mut post = gk.post1(&si, a)
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
