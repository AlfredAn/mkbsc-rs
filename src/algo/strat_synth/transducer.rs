use crate::*;

pub use strategy::*;
pub use strategy1::*;

pub fn transducer<'a, S: Strategy>
    (g: ConstructedGame<KBSC, 1>, s: S)
    -> AbstractTransducer<S> {
    AbstractTransducer::new(
        g,
        s
    )
}

#[derive(new, Debug, Clone)]
pub struct AbstractTransducer<S: Strategy> {
    gk: ConstructedGame<KBSC, 1>,
    strat: S // strategy for G^K
}

// impl strategy for G
impl<S: Strategy> Strategy for AbstractTransducer<S>
where
    S: Debug,
    S::M: Debug
{
    type M = (Option<(Loc, Act)>, S::M); // Loc in G^K

    fn call(&self, o_g: Obs, (la0, m): &Self::M) -> Option<(Act, Self::M)> {
        let g = &self.gk.origin().g;
        let gk = &self.gk;
        let strat: &S = self.strat.borrow();

        // map location in gk to observation in g
        let map_to_obs = |l_gk| {
            let l_g = self.gk.origin_loc(l_gk).first().unwrap();
            let [o_g] = g.observe(l_g);
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

// (obs, mem) -> option(act, mem)

#[derive(Debug, Clone, new)]
pub struct Transducer {
    pub tr: BTreeMap<(TransducerState, Obs), (TransducerState, Act)>
}

impl Transducer {
    pub fn build<S: Strategy>(g: &Game<1>, strat: &S) -> Self {
        let mut tr = BTreeMap::new();
        let mut state_map = HashMap::new();
        let mut empty = HashSet::new();
        let mut states = Vec::new();

        macro_rules! state {
            ($mem:expr) => {{
                let mem = $mem;
                match state_map.get(&mem) {
                    Some(&s) => s,
                    None => {
                        let s = transducer_state(states.len());
                        state_map.insert(mem.clone(), s);
                        states.push(mem);
                        s
                    }
                }
            }}
        }

        let mut stack = vec![(state!(strat.init()), g.l0())];

        while let Some((s, l)) = stack.pop() {
            let [o] = g.observe(l);

            if tr.contains_key(&(s, o)) || empty.contains(&(s, o)) {
                continue;
            }

            let m = &states[s.index()];
            if let Some((a, m2)) = strat.call(o, m) {
                let s2 = state!(m2);
                tr.insert((s, o), (s2, a));

                for l2 in g.post(l, [a]) {
                    stack.push((s2, l2));
                }
            } else {
                empty.insert((s, o));
            }
        }

        Transducer::new(tr)
    }
}
