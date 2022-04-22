use crate::*;

pub use strategy::*;
pub use strategy1::*;

// (obs, mem) -> option(act, mem)

#[derive(Debug, Clone)]
pub struct Transducer(BTreeMap<(Obs, TransducerState), (Act, TransducerState)>);

impl Strategy for Transducer {
    type M = TransducerState;

    fn call(&self, obs: Obs, &mem: &Self::M) -> Option<(Act, Self::M)> {
        self.0.get(&(obs, mem)).map(|&x| x)
    }

    fn init(&self) -> Self::M {
        transducer_state(0)
    }
}

impl Transducer {
    pub fn build<S: Strategy + ?Sized>(g: &Game<1>, strat: &S) -> Self {
        Self::build_ma(g, 0, strat)
    }

    pub fn build_ma<S: Strategy + ?Sized, const N: usize>(g: &Game<N>, agt: Agt, strat: &S) -> Self {
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

        let mut stack = vec![(g.l0(), state!(strat.init()))];

        while let Some((l, s)) = stack.pop() {
            let o = g.observe(l)[agt];

            if tr.contains_key(&(o, s)) || empty.contains(&(o, s)) {
                continue;
            }

            let m = &states[s.index()];
            if let Some((a, m2)) = strat.call(o, m) {
                let s2 = state!(m2);
                tr.insert((o, s), (a, s2));

                for &(a_succ, l2) in g.successors(l) {
                    if a == a_succ[agt] {
                        stack.push((l2, s2));
                    }
                }
            } else {
                empty.insert((o, s));
            }
        }

        Transducer(tr)
    }
}
