use std::rc::Rc;
use crate::{game::*, algo::*, dgame::*};

type K = KBSC<DGame<1>, Rc<DGame<1>>>;
type KLoc = <K as Game<1>>::Loc;

pub struct DKBSC {
    g: Rc<DGame<1>>,
    gk: K,
    dgk: Rc<DGame<1>>,

    s_map: Vec<KLoc>
}

impl DKBSC {
    fn new(g: Rc<DGame<1>>) -> Self {
        let gk = g.clone().kbsc();
        let dgk = Rc::new(gk.dgame().into_owned());

        let n = dgk.node_count();
        let mut s_map = Vec::with_capacity(n);
        /*explore(&*gk, |l| {
            s_map[l.index()] = Some()
        });*/

        Self {
            g, gk, dgk, s_map
        }
    }
}
