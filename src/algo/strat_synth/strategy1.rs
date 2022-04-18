use crate::*;
use Outcome::*;

type Depth = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub enum Outcome {
    Win(Depth), // guaranteed to win in at most x steps
    Maybe(Depth, Depth), // possible to win after x steps; possible to reach Win(_) state in y steps
    Lose
}

impl Outcome {
    pub fn is_win(self) -> bool { if let Win(_) = self {true} else {false} }
    pub fn is_maybe(self) -> bool { if let Maybe(_, _) = self {true} else {false} }
    pub fn is_lose(self) -> bool { self == Lose }
    pub fn can_win(self) -> bool { self != Lose }
    pub fn increment(self) -> Self { match self { Win(x) => Win(x+1), Maybe(x, y) => Maybe(x+1, y+1), Lose => Lose } }
}

impl Ord for Outcome {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match (self, rhs) {
            (Win(a), Win(b)) => a.cmp(b),
            (Win(_), _) => Ordering::Less,
            (_, Win(_)) => Ordering::Greater,
            (Maybe(a, b), Maybe(c, d)) => b.cmp(d).then(a.cmp(c)),
            (Maybe(_, _), Lose) => Ordering::Less,
            (Lose, Maybe(_, _)) => Ordering::Greater,
            (Lose, Lose) => Ordering::Equal
        }
    }
}

impl BitAnd for Outcome {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Win(a), Win(b)) => Win(max(a, b)),
            (Win(a), Maybe(b, _)) => Maybe(min(a, b), 0),
            (Win(a), Lose) => Maybe(a, 0),
            (Maybe(a, _), Win(b)) => Maybe(min(a, b), 0),
            (Maybe(a, b), Maybe(c, d)) => Maybe(min(a, c), min(b, d)),
            (Maybe(a, b), Lose) => Maybe(a, b),
            (Lose, Win(a)) => Maybe(a, 0),
            (Lose, Maybe(a, b)) => Maybe(a, b),
            (Lose, Lose) => Lose
        }
    }
}

impl BitOr for Outcome {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Win(a), Win(b)) => Win(min(a, b)),
            (Win(a), Maybe(_, _)) => Win(a),
            (Maybe(_, _), Win(a)) => Win(a),
            (Maybe(a, b), Maybe(c, d)) => Maybe(min(a, c), min(b, d)),
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
    fn insert(&mut self, a: Act, outcome: Outcome) -> (bool, Outcome) {
        let a = &mut self.action[a as usize];
        let a_old = *a;
        *a |= outcome;

        let l_old = self.outcome;
        self.outcome |= outcome;

        (*a != a_old || self.outcome != l_old, l_old)
    }
}

pub fn find_memoryless_strategies<T>(g: &Game<T, 1>) -> Vec<StratEntry> {
    let n = g.n_loc();

    let mut w = Vec::with_capacity(n);
    let mut w_list = Vec::new();

    let n_actions = g.n_actions[0];

    for l in (0..n).map(|l| loc(l)) {
        if g.is_winning(l) {
            w.push(StratEntry::new(n_actions, true));
            w_list.push(l);
        } else {
            w.push(StratEntry::new(n_actions, false));
        }
    }
    
    let mut buf = Vec::new();
    //let mut depth = 1;
    loop {
        //println!("depth={}", depth);

        for a in 0..n_actions {
            for l in g.pre_set(w_list.iter().copied(), [a]) {
                let outcome = g.post(l, [a])
                    .map(|l2| w[l2.index()].outcome)
                    //.inspect(|x| println!("    {:?}", x))
                    .reduce(|x, y| x & y)
                    .unwrap();
                
                //println!("  pre: {:?}", (l, a, outcome.increment()));
                buf.push((l, a, outcome.increment()));
            }
        }

        let mut updated = false;
        for (l, a, outcome) in buf.drain(..) {
            let (did_change, old) = w[l.index()].insert(a, outcome);
            if did_change {
                if outcome.can_win() && !old.can_win() {
                    w_list.push(l);
                }
                updated = true;
            }
        }
        if !updated { break; }

        //depth += 1;
    }

    w
}

#[derive(Debug, Clone)]
pub struct AllStrategies1<T> {
    strat: Vec<Option<Act>>,
    variables: Vec<(Loc<T>, Vec<Act>, u32)>
}

#[derive(new, Clone)]
pub struct MlessStrat<T, R: Borrow<Vec<Option<Act>>>>(R, PhantomData<T>);

impl<T, R: Borrow<Vec<Option<Act>>>> Strategy<T> for MlessStrat<T, R> {
    type M = ();

    fn call(&self, obs: Obs<T>, _: &()) -> Option<(Act, ())> {
        self.0.borrow()[obs.index()].map(|a| (a, ()))
    }

    fn init(&self) -> Self::M { () }
}

impl<T, R> Debug for MlessStrat<T, R>
where
    R: Borrow<Vec<Option<Act>>>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_list(f, self.0.borrow(), |f, x|
            if let Some(a) = x {
                write!(f, "{}", a)
            } else {
                write!(f, "-")
            }
        )
    }
}

impl<T> AllStrategies1<T> {
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

    pub fn get(&self) -> MlessStrat<T, Vec<Option<Act>>> {
        MlessStrat::new(self.get_raw().clone())
    }

    pub fn get_ref<'a>(&'a self) -> MlessStrat<T, &'a Vec<Option<Act>>> {
        MlessStrat::new(self.get_raw())
    }

    pub fn get_raw(&self) -> &Vec<Option<Act>> {
        &self.strat
    }

    pub fn reset(&mut self) {
        for (_, _, i) in &mut self.variables {
            *i = 0;
        }
    }

    pub(crate) fn new(w: &Vec<StratEntry>, n: usize) -> Self {
        let mut base = Vec::with_capacity(n);
        let mut variables = Vec::new();

        let mut buf = Vec::new();
        
        for (l, entry) in w.iter().enumerate() {
            for (a, &outcome) in entry.action.iter().enumerate() {
                if outcome.can_win() {
                    buf.push((a, outcome));
                }
            }

            if entry.outcome.is_win() {
                let best = buf.iter()
                    .copied()
                    .reduce(|(a1, o1), (a2, o2)| if o1 <= o2 {(a1, o1)} else {(a2, o2)})
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
                        loc(l),
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

pub fn all_strategies1<T>(g: &Game<T, 1>) -> AllStrategies1<T> {
    let fms = find_memoryless_strategies(g);
    //println!("{:#?}", fms);

    AllStrategies1::new(
        &fms,
        g.n_obs(0)
    )
}
