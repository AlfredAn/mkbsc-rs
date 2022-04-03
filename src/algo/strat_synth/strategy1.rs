
use std::collections::BTreeSet;
use std::cmp::*;
use std::ops::*;
use self::NodeStatus::*;
use crate::*;
use crate::game::*;

type Depth = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum NodeStatus {
    Win(Depth), // guaranteed to win in at most x steps
    Maybe(Depth, Depth), // possible to win after x steps; possible to reach Win(_) state in y steps
    Unknown
}

impl NodeStatus {
    pub fn is_win(self) -> bool { if let Win(_) = self {true} else {false} }
    pub fn is_either(self) -> bool { if let Maybe(_, _) = self {true} else {false} }
    pub fn is_known(self) -> bool { self != Unknown }
    pub fn is_unknown(self) -> bool { self == Unknown }
    pub fn can_win(self) -> bool { self.is_known() }
    pub fn increment(self) -> Self { match self { Win(x) => Win(x+1), Maybe(x, y) => Maybe(x+1, y+1), Unknown => Unknown } }
}

impl Ord for NodeStatus {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Win(a), Win(b)) => a.cmp(b),
            (Win(_), _) => Ordering::Less,
            (_, Win(_)) => Ordering::Greater,
            (Maybe(a, b), Maybe(c, d)) => b.cmp(d).then(a.cmp(c)),
            (Maybe(_, _), Unknown) => Ordering::Less,
            (Unknown, Maybe(_, _)) => Ordering::Greater,
            (Unknown, Unknown) => Ordering::Equal
        }
    }
}

impl BitAnd for NodeStatus {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Win(a), Win(b)) => Win(max(a, b)),
            (Win(a), Maybe(b, _)) => Maybe(min(a, b), 0),
            (Win(a), Unknown) => Maybe(a, 0),
            (Maybe(a, _), Win(b)) => Maybe(min(a, b), 0),
            (Maybe(a, b), Maybe(c, d)) => Maybe(min(a, c), min(b, d)),
            (Maybe(a, b), Unknown) => Maybe(a, b),
            (Unknown, Win(a)) => Maybe(a, 0),
            (Unknown, Maybe(a, b)) => Maybe(a, b),
            (Unknown, Unknown) => Unknown
        }
    }
}

impl BitOr for NodeStatus {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Win(a), Win(b)) => Win(min(a, b)),
            (Win(a), Maybe(_, _)) => Win(a),
            (Maybe(_, _), Win(a)) => Win(a),
            (Maybe(a, b), Maybe(c, d)) => Maybe(min(a, c), min(b, d)),
            (Unknown, x) => x,
            (x, Unknown) => x
        }
    }
}

impl BitAndAssign for NodeStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOrAssign for NodeStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl Default for NodeStatus {
    fn default() -> Self { Unknown }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StratEntry {
    action: Vec<NodeStatus>,
    status: NodeStatus
}

impl StratEntry {
    fn new(n: usize, is_goal: bool) -> Self {
        Self {
            action: vec![Unknown; n],
            status: if is_goal {Win(0)} else {Unknown}
        }
    }
    fn insert(&mut self, a: ActionIndex, status: NodeStatus) -> bool {
        self.action[a.index()] |= status;

        let old_status = self.status;
        self.status |= status;

        old_status != self.status
    }
}

pub fn find_memoryless_strategies<'a>(g: impl Game<'a, 1>) -> Vec<StratEntry> {
    let g = g.dgame();
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
    let mut depth = 1;
    loop {
        println!("depth={}", depth);

        for a in g.actions1() {
            for l in g.pre(w_list.iter().copied(), a) {
                let status = g.post1(&l, a)
                    .map(|l2| w[l2.index()].status)
                    .inspect(|x| println!("    {:?}", x))
                    .reduce(|x, y| x & y)
                    .unwrap();
                
                println!("  pre: {:?}", (l, a, status.increment()));
                buf.push((l, a, status.increment()));
            }
        }

        let mut inserted = false;
        for (l, a, status) in buf.drain(..) {
            if w[l.index()].insert(a, status) {
                println!("  push: {:?}", (l, a, status));
                w_list.push(l);
                inserted = true;
            }
        }
        if !inserted { break; }

        depth += 1;
    }

    w
}
