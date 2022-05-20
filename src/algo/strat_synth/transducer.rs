use crate::*;

pub use strategy::*;
pub use strategy1::*;

#[derive(Debug, Clone)]
pub struct Transducer(
    BTreeMap<(Obs, TransducerState), TransducerState>,
    Vec<Act>
);

impl Strategy for Transducer {
    type M = TransducerState;

    fn update(&self, obs: Obs, &mem: &Self::M) -> Option<Self::M> {
        self.0.get(&(obs, mem)).copied()
    }

    fn action(&self, mem: &Self::M) -> Act {
        self.1[mem.index()]
    }

    fn init(&self) -> Self::M {
        transducer_state(0)
    }
}

impl Transducer {
    pub fn build<S: Strategy + ?Sized>(g: &Game<1>, strat: &S) -> Self {
        Self::build_ma(g, agt(0), strat)
    }

    pub fn build_ma<S: Strategy + ?Sized, const N: usize>(g: &Game<N>, agt: Agt, strat: &S) -> Self {
        let mut transitions = BTreeMap::default();
        let mut actions = Vec::new();

        let mut state_map = FxHashMap::default();
        let mut visited = FxHashSet::default();
        let mut states = Vec::new();

        macro_rules! state {
            ($mem:expr) => {{
                let mem = $mem;
                match state_map.get(mem) {
                    Some(&s) => s,
                    None => {
                        let s = transducer_state(states.len());
                        state_map.insert(mem.clone(), s);
                        actions.push(strat.action(&mem));
                        states.push(mem.clone());
                        s
                    }
                }
            }}
        }

        let mut stack = vec![(g.l0(), state!(&strat.init()))];
        let mut first = true;

        while let Some((l1, s1)) = stack.pop() {
            if visited.contains(&(l1, s1)) {
                continue;
            }

            let m1 = &states[s1.index()];
            let o1 = g.observe(l1)[agt.index()];

            let m2 = if first {
                Some(m1.clone())
            } else {
                strat.update(o1, m1)
            };

            if let Some(m2) = m2 {
                let s2 = if first {
                    s1
                } else {
                    let s2 = state!(&m2);
                    transitions.insert((o1, s1), s2);
                    s2
                };

                let a = strat.action(&m2);

                for &(a_succ, l2) in g.successors(l1) {
                    if a == a_succ[agt.index()] {
                        stack.push((l2, s2));
                    }
                }
            }

            visited.insert((l1, s1));
            first = false;
        }

        Transducer(transitions, actions)
    }
}
