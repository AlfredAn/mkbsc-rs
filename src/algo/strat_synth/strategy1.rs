use crate::*;
use Outcome::*;
use itertools::izip;

type Depth = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl PartialOrd for Outcome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

#[derive(Clone, Debug)]
pub struct OutcomeSet {
    pub action: Vec<Outcome>,
    pub outcome: Outcome,
    pub initial: Outcome
}

impl OutcomeSet {
    fn new(n: usize, is_goal: bool) -> Self {
        let outcome = if is_goal {Win(0)} else {Lose};
        Self {
            action: vec![Lose; n],
            outcome,
            initial: outcome
        }
    }
 
    fn insert(&mut self, a: Act, outcome: Outcome) -> (bool, Outcome) {
        let a = &mut self.action[a.index()];
        let a_old = *a;
        *a |= outcome;

        let l_old = self.outcome;
        self.outcome |= outcome;

        (*a != a_old || self.outcome != l_old, l_old)
    }
    fn calculate_outcome(&mut self) {
        self.outcome = self.action.iter()
            .copied()
            .fold(self.initial, |o1, o2| o1 | o2);
    }
}

impl BitAndAssign<&Self> for OutcomeSet {
    fn bitand_assign(&mut self, rhs: &Self) {
        for (a, b) in izip!(&mut self.action, &rhs.action) {
            *a &= *b;
        }
        self.calculate_outcome();
    }
}

pub fn find_obs_outcomes(g: &Game<1>) -> Vec<OutcomeSet> {
    let sl = find_outcomes(g);
    // println!("pi: {:#?}", sl);

    let mut so = Vec::with_capacity(g.n_obs(agt(0)));

    for (_, set) in g.iter_obs(agt(0)) {
        let mut entry = sl[set[0].index()].clone();
        for l in set[1..].iter() {
            entry &= &sl[l.index()];
        }
        so.push(entry);
    }

    so
}

pub fn find_outcomes(g: &Game<1>) -> Vec<OutcomeSet> {
    let n = g.n_loc();

    let mut w = Vec::with_capacity(n);
    let mut w_list = Vec::new();

    let n_actions = g.n_actions[0];

    for l in (0..n).map(|l| loc(l)) {
        if g.is_winning(l) {
            w.push(OutcomeSet::new(n_actions, true));
            w_list.push(l);
        } else {
            w.push(OutcomeSet::new(n_actions, false));
        }
    }
    
    let mut buf = Vec::new();
    //let mut depth = ;
    loop {
        //println!("depth={}", depth);

        for a in (0..n_actions).map(|a| act(a)) {
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
/*
#[derive(Debug, Clone)]
pub struct AllStrategies1 {
    strat: Vec<Option<Act>>,
    variables: Vec<(Loc, Vec<Act>, u32)>
}

#[derive(new, Clone)]
pub struct MlessStrat(Vec<Option<Act>>);

impl Strategy for MlessStrat {
    type M = ();

    fn call(&self, obs: Obs, _: &()) -> Option<(Act, ())> {
        self.0[obs.index()].map(|a| (a, ()))
    }

    fn init(&self) -> () { () }
}

impl Debug for MlessStrat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_list(f, self.0.iter(), |f, x|
            if let Some(a) = x {
                write!(f, "{}", a)
            } else {
                write!(f, "-")
            }
        )
    }
}

impl AllStrategies1 {
    pub fn count(&self) -> u128 {
        self.variables.iter()
            .map(|(_, v, _)| v.len() as u128)
            .fold(1, |a, b| a * b)
    }

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

    pub fn get(&self) -> MlessStrat {
        MlessStrat::new(self.get_raw().clone())
    }

    pub fn get_raw(&self) -> &Vec<Option<Act>> {
        &self.strat
    }

    pub fn reset(&mut self) {
        for (_, _, i) in &mut self.variables {
            *i = 0;
        }
    }

    pub fn new(w: &Vec<OutcomeSet>, g: &Game<1>) -> Self {
        let mut strat = VecMap::with_capacity(g.n_obs(agt(0)));
        let mut visited = LocSet::new(g);
        let mut added = LocSet::new(g);
        let mut queue = VecDeque::new();

        queue.push_back(g.l0());
        added.insert(g.l0());

        let mut buf = Vec::new();

        while let Some(l) = queue.pop_front() {
            visited.insert(l);
            let [obs] = g.observe(l);

            if g.is_obs_winning(agt(0), obs) {
                continue;
            }

            let mut itr = g.obs_set(agt(0), obs)
                .iter()
                .filter(|&&l| visited.contains(l))
                .map(|l| &w[l.index()]);
            let outcomes = itr.fold(
                itr.next().unwrap().clone(),
                |e, e2| {
                    e &= e2;
                    e
                }
            );

            assert!(buf.is_empty());
            for (a, &outcome) in outcomes.action.iter().enumerate() {
                if outcome.can_win() {
                    buf.push((act(a), outcome));
                }
            }

            if outcomes.outcome.is_win() {
                let best = buf.iter()
                    .copied()
                    .reduce(|(a1, o1), (a2, o2)| if o1 <= o2 {(a1, o1)} else {(a2, o2)});
                buf.clear();

                if let Some(best) = best {
                    buf.push(best);
                }
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
                            .map(|(a, _)| act(a))
                            .collect(),
                        0
                    ));
                }
            }
        }
        todo!();

        AllStrategies1 {
            strat: base,
            variables: variables
        }
    }

    pub fn into_iter(mut self) -> impl Iterator<Item=MlessStrat> {
        let mut first = true;
        let mut done = false;
        iter::from_fn(move || {
            if first {
                first = false;
                Some(self.get())
            } else {
                if !done && self.advance() {
                    Some(self.get())
                } else {
                    done = true;
                    None
                }
            }
        })
    }
}

pub fn all_strategies1(g: &Game<1>) -> AllStrategies1 {
    // let fms = find_memoryless_strategies(g);
    todo!()

    // println!("{}", g);
    // println!("ii: {:#?}", fms);

    // AllStrategies1::new(
    //     &fms,
    //     g.n_obs(agt(0))
    // )
}
*/