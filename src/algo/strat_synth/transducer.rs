use crate::*;

pub use strategy::*;
pub use strategy1::*;

#[derive(Clone)]
pub struct Transducer {
    transitions: FxHashMap<(Obs, TransducerState), TransducerState>,
    actions: Vec<Act>,
    origin: Rc<dyn Origin>,
    agt: Agt
}

impl Debug for Transducer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tr = display(|f|
            format_sep(
                f, ",\n    ",
                self.transitions.iter().sorted_unstable_by_key(|(&(o, m1), &m2)| (m1, o, m2)),
                |f, (&(o, m1), m2)| {
                    let o = display(|f| self.origin.fmt_obs(f, self.agt, o));
                    write!(f, "s{m1} -> ({o}) -> s{m2}")
                }
            )
        );

        let act = display(|f|
            format_sep(
                f, ",\n    ",
                self.actions.iter().enumerate(),
                |f, (i, a)|
                    write!(f, "s{i}: a{a}")
            )
        );

        write!(f, "Transducer {{\n    {tr};\n    {act}\n}}")
    }
}

impl Strategy for Transducer {
    type M = TransducerState;

    fn update(&self, obs: Obs, &mem: &Self::M) -> Option<Self::M> {
        self.transitions.get(&(obs, mem)).copied()
    }

    fn action(&self, mem: &Self::M) -> Act {
        self.actions[mem.index()]
    }

    fn init(&self) -> Self::M {
        transducer_state(0)
    }
}

impl Transducer {
    pub fn build<S: Strategy + ?Sized>(g: Rc<Game<1>>, strat: &S) -> Self {
        Self::build_ma(g, agt(0), strat)
    }

    pub fn build_ma<S: Strategy + ?Sized, const N: usize>(g: Rc<Game<N>>, agt: Agt, strat: &S) -> Self {
        let mut transitions = FxHashMap::default();
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

        // eprintln!("--------------------------------------------------");

        while let Some((l1, s1)) = stack.pop() {
            if visited.contains(&(l1, s1)) {
                continue;
            }

            // eprintln!("visit: (({l1}){}, s{:?})", display(|f| g.fmt_loc(f, l1)), s1);

            let m1 = states[s1.index()].clone();
            let a = strat.action(&m1);

            // eprintln!("  a={a}");

            for &(a_succ, l2) in g.successors(l1) {
                if a == a_succ[agt.index()] {
                    // eprintln!("    post: ({l2}){}", display(|f| g.fmt_loc(f, l2)));
                    
                    let o2 = g.observe(l2)[agt.index()];

                    // eprintln!("      obs: ({o2}){}", display(|f| g.fmt_obs(f, agt, o2)));

                    if let Some(m2) = strat.update(o2, &m1) {
                        let s2 = state!(&m2);
                        // eprintln!("    -> s{s2}");
                        
                        transitions.insert((o2, s1), s2);
                        stack.push((l2, s2));
                    }
                }
            }

            visited.insert((l1, s1));
        }

        Transducer { transitions, actions, origin: g, agt }
    }
}
