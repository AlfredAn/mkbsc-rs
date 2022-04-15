use slice_group_by::GroupBy;
use crate::*;

pub type Loc = u32;
pub type Obs = u32;

#[derive(Clone, SmartDefault)]
pub struct Game<T, const N: usize> {
    pub loc: Vec<LocData<T, N>>,

    #[default([0; N])]
    pub n_actions: [usize; N],

    #[default(array_init(|_| Vec::new()))]
    pub obs: [Vec<Vec<Loc>>; N]
}

#[derive(Debug, Clone)]
pub struct LocData<T, const N: usize> {
    // sorted by action
    pub successors: Vec<([Act; N], Loc)>, 
    pub predecessors: Vec<([Act; N], Loc)>,
    
    pub is_winning: bool,
    pub obs: [Obs; N],
    pub obs_offset: [usize; N],
    pub data: T
}

impl<T, const N: usize> Game<T, N> {
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
    pub fn data(&self, l: Loc) -> &T {
        &self[l].data
    }

    pub fn obs_set(&self, agt: Agt, obs: Obs) -> &[Loc] {
        &self.obs[agt][obs as usize]
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

    pub fn iter(&self) -> impl Iterator<Item=(Loc, &LocData<T, N>)> {
        self.loc.iter().enumerate().map(|(i, x)| (i as Loc, x))
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
        0
    }
}

impl<T, const N: usize> Index<Loc> for Game<T, N> {
    type Output = LocData<T, N>;
    fn index(&self, l: Loc) -> &LocData<T, N> { &self.loc[l as usize] }
}
impl<T, const N: usize> IndexMut<Loc> for Game<T, N> {
    fn index_mut(&mut self, l: Loc) -> &mut LocData<T, N> { &mut self.loc[l as usize] }
}

macro_rules! impl_format {
    ($trait:path, $fmt:literal, $fmt2:literal, $fmt3:literal) => {
        impl<T: $trait, const N: usize> $trait for Game<T, N> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let ns = self.iter().format_with(",\n           ", |(_, n), f| {
                    f(&format_args!($fmt2, n.data, if n.is_winning {":W"} else {""}))
                });
        
                let os = self.obs.iter().enumerate().format_with("\n    ", |(i, oi), f|
                    f(&format_args!("Obs[{}]: {{ {} }}",
                        i,
                        oi.iter().format_with(",\n              ", |o, f|
                            f(&format_args!("{}",
                                o.iter().format_with("|", |&l, f|
                                    f(&format_args!($fmt, self.data(l)))
                                )
                            ))
                        )
                    ))
                );
                
                let es = self.edges()
                    .format_with(",\n             ", |(l, a, l2), f| {
                        f(&format_args!($fmt3,
                            self.data(l),
                            self.data(l2),
                            a.iter().format(".")
                        ))
                    });
        
                write!(f, "Game {{\n")?;
                write!(f, "    n_agents: {}, n_actions: {:?}\n", N, self.n_actions)?;
                write!(f, "    Loc: {{ {} }}\n    {}\n    Delta: {{ {} }}\n", ns, os, es)?;
                write!(f, "}}")
            }
        }
    };
}

impl_format!(fmt::Debug, "{:?}", "{:?}{}", "({:?}->{:?}, {})");
impl_format!(fmt::Display, "{}", "{}{}", "({}->{}, {})");
