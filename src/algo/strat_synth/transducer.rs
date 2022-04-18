use crate::*;

pub use strategy::*;
pub use strategy1::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Gray,
    Black
}

#[derive(Debug, Clone)]
enum StackEntry<T, M> {
    Visit(Loc<T>, M),
    Finish(Loc<T>, M)
}

pub fn transducer<'a, T: Clone, S: Strategy<KBSCData<T>>>
    (g: ConstructedGame<KBSC<T>, 1>, s: S)
    -> AbstractTransducer<T, S> {
    AbstractTransducer::new(
        g,
        s
    )
}

#[derive(new, Debug, Clone)]
pub struct AbstractTransducer<T, S: Strategy<KBSCData<T>>> {
    gk: ConstructedGame<KBSC<T>, 1>,
    strat: S // strategy for G^K
}

// impl strategy for G
impl<T, S: Strategy<KBSCData<T>>> Strategy<T> for AbstractTransducer<T, S>
where
    S: Debug,
    S::M: Debug,
    T: Debug
{
    type M = (Option<(KLoc<T>, Act)>, S::M); // Loc in G^K

    fn call(&self, o_g: Obs<T>, (la0, m): &Self::M) -> Option<(Act, Self::M)> {
        let gk = &self.gk.game;
        let strat: &S = self.strat.borrow();

        // map location in gk to observation in g
        let map_to_obs = |l_gk| {
            let d = gk.data(l_gk);
            //println!("        data={:?}", d.g);

            let o_g = d.obs;
            //println!("        o_g: {:?}", o_g);
            o_g
        };

        //println!("    o_g={:?}", o_g);

        let l_gk = if let &Some((l0_gk, a0)) = la0 {
            //println!("    l0_gk={:?}, a0={:?}", l0_gk, a0);

            //println!("    {:?}", gk.post_raw(l0_gk, [a0]));

            let mut post = gk.post(l0_gk, [a0])
                //.inspect(|&l_gk| println!("      post_gk: {:?}", l_gk))
                .filter(|&l_gk| map_to_obs(l_gk) == o_g);
            
            if let Some(l_gk) = post.next() {
                assert!(post.next().is_none()); // should be guaranteed by theory
                l_gk
            } else {
                //println!("    l_gk=None");
                return None;
            }
        } else {
            gk.l0()
        };

        let [o_gk] = gk.observe(l_gk);
        //println!("    l_gk={:?}, o_gk={:?}", l_gk, o_gk);

        //println!("    {:?}", strat);

        if let Some((a, m2)) = strat.call(o_gk, &m) {
            //println!("    a={:?}, m2={:?}", a, m2);
            return Some((a, (Some((l_gk, a)), m2)));
        } else {
            //println!("    strat returned None");
        }
        
        None
    }

    fn init(&self) -> Self::M {
        (None, self.strat.borrow().init())
    }
}
