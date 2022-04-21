use slice_group_by::GroupBy;
use crate::*;

#[derive(Clone, SmartDefault)]
pub struct Game<const N: usize> {
    pub loc: Vec<LocData<N>>,

    #[default([0; N])]
    pub n_actions: [usize; N],

    #[default(array_init(|_| Vec::new()))]
    pub obs: [Vec<Vec<Loc>>; N],

    pub origin: Option<Rc<dyn Origin>>
}

#[derive(Debug, Clone)]
pub struct LocData<const N: usize> {
    // sorted by action
    pub successors: Vec<([Act; N], Loc)>, 
    pub predecessors: Vec<([Act; N], Loc)>,
    
    pub is_winning: bool,
    pub obs: [Obs; N],
    pub obs_offset: [usize; N]
}

impl<const N: usize> Game<N> {
    pub fn n_loc(&self) -> usize {
        self.loc.len()
    }
    pub fn n_obs(&self, agt: Agt) -> usize {
        self.obs[agt as usize].len()
    }

    pub fn observe(&self, l: Loc) -> [Obs; N] {
        self[l].obs
    }
    pub fn obs_offset(&self, l: Loc) -> [usize; N] {
        self[l].obs_offset
    }
    pub fn is_winning(&self, l: Loc) -> bool {
        self[l].is_winning
    }

    pub fn obs_set(&self, agt: Agt, obs: Obs) -> &[Loc] {
        &self.obs[agt][obs.index()]
    }

    pub fn n_action_profiles(&self) -> usize {
        self.n_actions.iter().product()
    }
    pub fn action_profiles(&self) -> impl Iterator<Item=[Act; N]> {
        range_product(self.n_actions.map(|n| 0..n))
            .map(|aa| aa.map(|a| a as Act))
    }
    fn action_profile_index(&self, a: [Act; N]) -> usize {
        let mut result = 0;
        for i in (0..N).rev() {
            result *= self.n_actions[i];
            result += a[i];
        }
        result
    }

    pub fn iter(&self) -> impl Iterator<Item=(Loc, &LocData<N>)> {
        self.loc.iter().enumerate().map(|(i, x)| (loc(i), x))
    }
    pub fn iter_obs(&self, agt: Agt) -> impl Iterator<Item=(Obs, &[Loc])> {
        self.obs[agt].iter().enumerate().map(|(i, x)| (obs(i), &**x))
    }
    pub fn iter_agt(&self) -> impl Iterator<Item=Agt> {
        0..N
    }

    pub fn edges(&self) -> impl Iterator<Item=(Loc, [Act; N], Loc)> + '_ {
        self.iter()
            .flat_map(|(l, d)|
                d.successors.iter()
                    .map(move |&(a, l2)| (l, a, l2))
            )
    }

    pub fn successors(&self, l: Loc) -> &[([Act; N], Loc)] {
        &self[l].successors
    }
    pub fn post_raw(&self, l: Loc, a: [Act; N]) -> &[([Act; N], Loc)] {
        self.successors(l)
            .linear_group_by(|(a1, _), (a2, _)| a1 == a2)
            .find(|slice| slice[0].0 == a)
            .unwrap_or(&[])
    }
    pub fn post(&self, l: Loc, a: [Act; N]) -> impl Iterator<Item=Loc> + '_ {
        self.post_raw(l, a).iter().map(|&(_, l)| l)
    }
    pub fn post_set<'a, I>(&'a self, s: I, a: [Act; N]) -> impl Iterator<Item=Loc> + 'a
    where
        I: IntoIterator<Item=Loc>,
        I::IntoIter: 'a
    {
        s.into_iter()
            .flat_map(move |l|
                self.post(l, a)
            )
    }

    pub fn predecessors(&self, l: Loc) -> &[([Act; N], Loc)] {
        &self[l].predecessors
    }
    pub fn pre_raw(&self, l: Loc, a: [Act; N]) -> &[([Act; N], Loc)] {
        self.predecessors(l)
            .linear_group_by(|(a1, _), (a2, _)| a1 == a2)
            .find(|slice| slice[0].0 == a)
            .unwrap_or(&[])
    }
    pub fn pre(&self, l: Loc, a: [Act; N]) -> impl Iterator<Item=Loc> + '_ {
        self.pre_raw(l, a).iter().map(|&(_, l)| l)
    }
    pub fn pre_set<'a, I>(&'a self, s: I, a: [Act; N]) -> impl Iterator<Item=Loc> + 'a
    where
        I: IntoIterator<Item=Loc>,
        I::IntoIter: 'a
    {
        s.into_iter()
            .flat_map(move |l|
                self.pre(l, a)
            )
    }

    pub fn l0(&self) -> Loc {
        loc(0)
    }

    pub fn to_unique_loc(&self, obs: Obs, agt: Agt) -> Option<Loc> {
        if self.obs[agt][obs.index()].len() == 1 {
            Some(self.obs[agt][obs.index()][0])
        } else {
            None
        }
    }

    pub fn fmt_loc(&self, f: &mut fmt::Formatter, l: Loc) -> fmt::Result {
        if let Some(origin) = &self.origin {
            origin.fmt_loc(f, l)
        } else {
            write!(f, "{}", l)
        }
    }

    pub fn fmt_obs(&self, f: &mut fmt::Formatter, agt: Agt, obs: Obs) -> fmt::Result {
        format_sep(f, " | ", self.obs_set(agt, obs).iter(), |f, &l|
            self.fmt_loc(f, l)
        )
    }
}

impl<const N: usize> Index<Loc> for Game<N> {
    type Output = LocData<N>;
    fn index(&self, l: Loc) -> &LocData<N> { &self.loc[l.index()] }
}
impl<const N: usize> IndexMut<Loc> for Game<N> {
    fn index_mut(&mut self, l: Loc) -> &mut LocData<N> { &mut self.loc[l.index()] }
}


impl<const N: usize> Debug for Game<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Game {{\n")?;
        write!(f, "    n_agents: {}, n_actions: {:?},\n", N, self.n_actions)?;
        write!(f, "    Loc: {{ ")?;
        format_sep(f, ",\n           ", self.iter(), |f, (l, _)|
            self.fmt_loc(f, l)
        )?;

        for agt in self.iter_agt() {
            write!(f, " }},\n    Obs[{}]: {{ ", agt)?;
            format_sep(f, ",\n              ", self.iter_obs(agt), |f, (o, _)|
                self.fmt_obs(f, agt, o)
            )?;
        }
        
        write!(f, " }},\n    Delta: {{ ")?;
        format_sep(f, ",\n             ", self.edges(), |f, (l, a, l2)| {
            format_sep(f, ".", a.iter(), |f, a|
                write!(f, "{}", a)
            )?;
            write!(f, " : ")?;
            self.fmt_loc(f, l)?;
            write!(f, " -> ")?;
            self.fmt_loc(f, l2)
        })?;

        write!(f, " }}\n}}")
    }
}
