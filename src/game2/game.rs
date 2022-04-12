use super::*;

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
    pub succ: Vec<([Act; N], Loc)>,
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

    pub fn iter(&self) -> impl Iterator<Item=(Loc, &LocData<T, N>)> {
        self.loc.iter().enumerate().map(|(i, x)| (i as Loc, x))
    }
    pub fn edges<'a>(&'a self) -> impl Iterator<Item=(Loc, [Act; N], Loc)> + 'a {
        self.iter()
            .flat_map(|(l, d)|
                d.succ.iter()
                    .map(move |&(a, l2)| (l, a, l2))
            )
    }

    pub fn succ(&self, l: Loc) -> &[([Act; N], Loc)] {
        &self[l].succ
    }
}

impl<T, const N: usize> Index<Loc> for Game<T, N> {
    type Output = LocData<T, N>;
    fn index(&self, l: Loc) -> &LocData<T, N> { &self.loc[l as usize] }
}
impl<T, const N: usize> IndexMut<Loc> for Game<T, N> {
    fn index_mut(&mut self, l: Loc) -> &mut LocData<T, N> { &mut self.loc[l as usize] }
}

impl<G: AbstractGame<N> + ?Sized, const N: usize> From<&G> for Game<G::Data, N> {
    fn from(g: &G) -> Self {
        let mut r = Game::default();
        r.n_actions = g.n_actions();

        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        let mut obs_map = HashMap::new();

        macro_rules! visit {
            ($l:expr) => {
                {
                    let l = $l;

                    let n = r.n_loc() as Loc;

                    let o = g.obs(&l);

                    let mut obs = ArrayVec::<_, N>::new();
                    let mut obs_offset = ArrayVec::<_, N>::new();
                    for (agt, oi) in o.into_iter().enumerate() {
                        let obs_i = match obs_map.entry((agt, oi)) {
                            Vacant(e) => {
                                let obs_i = r.obs[agt].len() as Obs;
                                r.obs[agt].push(Vec::new());
                                e.insert(obs_i);
                                obs_i
                            },
                            Occupied(e) => {
                                *e.get()
                            }
                        };
                        let obs_set = &mut r.obs[agt][obs_i as usize];

                        obs.push(obs_i);
                        obs_offset.push(obs_set.len());

                        obs_set.push(n);
                    }

                    r.loc.push(LocData {
                        succ: Vec::new(),
                        is_winning: g.is_winning(&l),
                        obs: (*obs).try_into().unwrap(),
                        obs_offset: (*obs_offset).try_into().unwrap(),
                        data: g.data(&l)
                    });

                    queue.push_back(l.clone());
                    visited.insert(l, n);

                    n
                }
            }
        }

        visit!(g.l0());

        let mut i = 0;
        while let Some(l) = queue.pop_front() {
            let n = i as Loc;

            g.succ(&l, |a, l2| {
                let n2 = if let Some(&n2) = visited.get(&l2) {
                    n2
                } else {
                    visit!(l2)
                };
                if !r.succ(n).iter().any(|&(a_, n_)| (a, n2) == (a_, n_)) {
                    r[n].succ.push((a, n2));
                }
            });

            i += 1;
        }

        r
    }
}

macro_rules! impl_format {
    ($trait:path, $fmt:literal, $fmt2:literal, $fmt3:literal) => {
        impl<T: $trait, const N: usize> $trait for Game<T, N> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let ns = self.iter().format_with(", ", |(_, n), f| {
                    f(&format_args!($fmt2, n.data, if n.is_winning {":W"} else {""}))
                });
        
                let os = self.obs.iter().enumerate().format_with("\n    ", |(i, oi), f|
                    f(&format_args!("Obs[{}]: [{}]",
                        i,
                        oi.iter().format_with(", ", |o, f|
                            f(&format_args!("{{{}}}",
                                o.iter().format_with("|", |&l, f|
                                    f(&format_args!($fmt, self.data(l)))
                                )
                            ))
                        )
                    ))
                );
                
                let es = self.edges()
                    .format_with(", ", |(l, a, l2), f| {
                        f(&format_args!($fmt3,
                            self.data(l),
                            self.data(l2),
                            a.iter().format(".")
                        ))
                    });
        
                write!(f, "Game {{\n")?;
                write!(f, "    n_agents: {}, n_actions: {:?}\n", N, self.n_actions)?;
                write!(f, "    Loc: [{}]\n    {}\n    Delta: [{}]\n", ns, os, es)?;
                write!(f, "}}")
            }
        }
    };
}

impl_format!(fmt::Debug, "{:?}", "{:?}{}", "({:?}->{:?}, {})");
impl_format!(fmt::Display, "{}", "{}{}", "({}->{}, {})");
