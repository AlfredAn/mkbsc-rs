
use crate::game::index::node_index;
use crate::game::dgame::DGame;
use std::cmp::*;
use std::ops::*;
use std::iter;
use Outcome::*;
use crate::game::*;

type Depth = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum Outcome {
    Win(Depth), // guaranteed to win in at most x steps
    Either(Depth, Depth), // possible to win after x steps; possible to reach Win(_) state in y steps
    Lose
}

impl Outcome {
    pub fn is_win(self) -> bool { if let Win(_) = self {true} else {false} }
    pub fn is_either(self) -> bool { if let Either(_, _) = self {true} else {false} }
    pub fn is_lose(self) -> bool { self == Lose }
    pub fn can_win(self) -> bool { self != Lose }
    pub fn increment(self) -> Self { match self { Win(x) => Win(x+1), Either(x, y) => Either(x+1, y+1), Lose => Lose } }
}

impl Ord for Outcome {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Win(a), Win(b)) => a.cmp(b),
            (Win(_), _) => Ordering::Less,
            (_, Win(_)) => Ordering::Greater,
            (Either(a, b), Either(c, d)) => b.cmp(d).then(a.cmp(c)),
            (Either(_, _), Lose) => Ordering::Less,
            (Lose, Either(_, _)) => Ordering::Greater,
            (Lose, Lose) => Ordering::Equal
        }
    }
}

impl BitAnd for Outcome {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Win(a), Win(b)) => Win(max(a, b)),
            (Win(a), Either(b, _)) => Either(min(a, b), 0),
            (Win(a), Lose) => Either(a, 0),
            (Either(a, _), Win(b)) => Either(min(a, b), 0),
            (Either(a, b), Either(c, d)) => Either(min(a, c), min(b, d)),
            (Either(a, b), Lose) => Either(a, b),
            (Lose, Win(a)) => Either(a, 0),
            (Lose, Either(a, b)) => Either(a, b),
            (Lose, Lose) => Lose
        }
    }
}

impl BitOr for Outcome {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Win(a), Win(b)) => Win(min(a, b)),
            (Win(a), Either(_, _)) => Win(a),
            (Either(_, _), Win(a)) => Win(a),
            (Either(a, b), Either(c, d)) => Either(min(a, c), min(b, d)),
            (Lose, x) => x,
            (x, Lose) => x
        }
    }
}

impl BitAndAssign for Outcome {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOrAssign for Outcome {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl Default for Outcome {
    fn default() -> Self { Lose }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StratEntry {
    pub action: Vec<Outcome>,
    pub outcome: Outcome
}

impl StratEntry {
    fn new(n: usize, is_goal: bool) -> Self {
        Self {
            action: vec![Lose; n],
            outcome: if is_goal {Win(0)} else {Lose}
        }
    }
    fn insert(&mut self, a: ActionIndex, outcome: Outcome) -> bool {
        self.action[a.index()] |= outcome;

        let old_outcome = self.outcome;
        self.outcome |= outcome;

        old_outcome != self.outcome
    }
}

pub fn find_memoryless_strategies(g: &DGame<1>) -> Vec<StratEntry> {
    let n = g.graph.node_count();

    let mut w = Vec::with_capacity(n);
    let mut w_list = Vec::new();

    for i in (0..n).map(|i| node_index(i)) {
        if g.is_winning(&i) {
            w.push(StratEntry::new(g.n_actions, true));
            w_list.push(i);
        } else {
            w.push(StratEntry::new(g.n_actions, false));
        }
    }
    
    let mut buf = Vec::new();
    //let mut depth = 1;
    loop {
        //println!("depth={}", depth);

        for a in g.actions1() {
            for l in g.pre(w_list.iter().copied(), a) {
                let outcome = g.post1(&l, a)
                    .map(|l2| w[l2.index()].outcome)
                    //.inspect(|x| //println!("    {:?}", x))
                    .reduce(|x, y| x & y)
                    .unwrap();
                
                //println!("  pre: {:?}", (l, a, outcome.increment()));
                buf.push((l, a, outcome.increment()));
            }
        }

        let mut inserted = false;
        for (l, a, outcome) in buf.drain(..) {
            if w[l.index()].insert(a, outcome) {
                //println!("  push: {:?}", (l, a, outcome));
                w_list.push(l);
                inserted = true;
            }
        }
        if !inserted { break; }

        //depth += 1;
    }

    w
}

#[derive(Debug, Clone)]
pub struct AllStrategies1 {
    strat: Vec<Option<ActionIndex>>,
    variables: Vec<(NodeIndex, Vec<ActionIndex>, u32)>
}

impl AllStrategies1 {
    pub fn advance(&mut self) -> bool {
        for (l, actions, i) in &mut self.variables {
            *i += 1;

            if (*i as usize) < actions.len() {
                self.strat[l.index()] = Some(actions[*i as usize]);
                return true;
            } else {
                *i = 0;
                self.strat[l.index()] = Some(actions[0]);
            }
        }

        false
    }

    pub fn get(&self) -> &Vec<Option<ActionIndex>> {
        &self.strat
    }

    pub fn reset(&mut self) {
        for (_, _, i) in &mut self.variables {
            *i = 0;
        }
    }

    pub fn iter<'b>(&'b mut self) -> impl Iterator<Item=Vec<Option<ActionIndex>>> + 'b {
        let mut finished = false;
        let mut first = true;
        iter::from_fn(move || {
            if finished {
                return None;
            } else if !first {
                if !self.advance() {
                    finished = true;
                    return None;
                }
            } else {
                first = false;
            }
            Some(self.get().clone())
        })
    }

    pub(crate) fn new(w: &Vec<StratEntry>, n: usize) -> Self {
        let mut base = Vec::with_capacity(n);
        let mut variables = Vec::new();

        let mut buf = Vec::new();
        
        for (l, entry) in w.iter().enumerate() {
            for (a, &outcome) in entry.action.iter().enumerate() {
                if outcome.can_win() {
                    buf.push((a.into(), outcome));
                }
            }

            if entry.outcome.is_win() {
                let best = buf.iter()
                    .copied()
                    .reduce(|(l1, o1), (l2, o2)| if o1 <= o2 {(l1, o1)} else {(l2, o2)})
                    .unwrap();
                buf.clear();
                buf.push(best);
            }

            match buf.len() {
                0 => base.push(None),
                1 => {
                    base.push(Some(buf[0].0));
                    buf.clear();
                },
                _ => {
                    buf.sort_by(|(_, o1), (_, o2)| o1.cmp(o2));
                    base.push(Some(buf[0].0));
                    
                    variables.push((
                        node_index(l),
                        buf.drain(..)
                            .map(|(a, _)| a)
                            .collect(),
                        0
                    ));
                }
            }
        }

        AllStrategies1 {
            strat: base,
            variables: variables
        }
    }
}
