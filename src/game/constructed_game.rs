use crate::*;
use collections::hash_map::Entry::*;

#[derive(Clone)]
pub struct ConstructedGame<G, const N: usize>
where
    G: AbstractGame<N> + ?Sized
{
    pub game: Rc<Game<N>>,
    origin: Rc<OriginImpl<G, N>>,
}

#[derive(Clone, Debug, new)]
struct OriginImpl<G, const N: usize>
where
    G: AbstractGame<N> + ?Sized
{
    game: Rc<G>,

    loc_map: FxHashMap<G::Loc, Loc>,
    obs_map: FxHashMap<(Agt, G::Obs), Obs>,
    
    loc_map_reverse: Vec<G::Loc>,
    obs_map_reverse: [Vec<G::Obs>; N]
}

pub trait Origin {
    fn fmt_loc(&self, f: &mut fmt::Formatter, l: Loc) -> fmt::Result;
    fn fmt_obs(&self, f: &mut fmt::Formatter, agt: Agt, o: Obs) -> fmt::Result;
}

impl<const N: usize> Origin for Game<N> {
    fn fmt_loc(&self, f: &mut fmt::Formatter, l: Loc) -> fmt::Result {
        self.fmt_loc(f, l)
    }

    fn fmt_obs(&self, f: &mut fmt::Formatter, agt: Agt, o: Obs) -> fmt::Result {
        self.fmt_obs(f, agt, o)
    }
}

impl<G, const N: usize> Origin for OriginImpl<G, N>
where
    G: AbstractGame<N> + ?Sized + 'static
{
    fn fmt_loc(&self, f: &mut fmt::Formatter, l: Loc) -> fmt::Result {
        self.game.fmt_loc(f, self.origin_loc(l))
    }

    fn fmt_obs(&self, f: &mut fmt::Formatter, agt: Agt, o: Obs) -> fmt::Result {
        self.game.fmt_obs(f, agt, self.origin_obs(agt, o))
    }
}

impl<G, const N: usize> OriginImpl<G, N>
where
    G: AbstractGame<N> + ?Sized
{
    fn origin_loc(&self, l: Loc) -> &G::Loc {
        &self.loc_map_reverse[l.index()]
    }
    fn origin_obs(&self, agt: Agt, o: Obs) -> &G::Obs {
        &self.obs_map_reverse[agt.index()][o.index()]
    }

    fn loc(&self, l: &G::Loc) -> Loc {
        self.loc_map[l]
    }
    fn obs(&self, key: &(Agt, G::Obs)) -> Obs {
        self.obs_map[key]
    }
}

impl<G, const N: usize> ConstructedGame<G, N>
where
    G: AbstractGame<N> + ?Sized
{
    pub fn origin(&self) -> &Rc<G> {
        &self.origin.game
    }

    pub fn origin_loc(&self, l: Loc) -> &G::Loc {
        self.origin.origin_loc(l)
    }
    pub fn origin_obs(&self, agt: Agt, o: Obs) -> &G::Obs {
        self.origin.origin_obs(agt, o)
    }

    pub fn loc(&self, l: &G::Loc) -> Loc {
        self.origin.loc(l)
    }
    pub fn obs(&self, key: &(Agt, G::Obs)) -> Obs {
        self.origin.obs(key)
    }
}

impl<G, const N: usize> Debug for ConstructedGame<G, N>
where
    G: AbstractGame<N> + ?Sized + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.origin())
    }
}

impl<G, const N: usize> Display for ConstructedGame<G, N>
where
    G: AbstractGame<N> + ?Sized
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.game)
    }
}


impl<G, const N: usize> Deref for ConstructedGame<G, N>
where
    G: AbstractGame<N> + ?Sized
{
    type Target = Rc<Game<N>>;
    fn deref(&self) -> &Self::Target {
        &self.game
    }
}

pub fn build_game<G, const N: usize>(g: Rc<G>, keep_origin: bool) -> ConstructedGame<G, N>
where
    G: AbstractGame<N> + ?Sized + 'static
{
    let mut r = Game::default();
    r.n_actions = g.n_actions();

    let mut queue = VecDeque::new();

    let mut loc_map = FxHashMap::default();
    let mut obs_map = FxHashMap::default();

    let mut loc_map_reverse = Vec::new();
    let mut obs_map_reverse = array_init(|_| Vec::new());

    macro_rules! visit {
        ($l:expr) => {
            {
                let l = $l;

                let n = loc(r.n_loc());

                let o = g.obs(&l);

                let mut obs_ = ArrayVec::<_, N>::new();
                let mut obs_offset = ArrayVec::<_, N>::new();
                
                for (i, oi) in o.into_iter().enumerate() {
                    let obs_i = match obs_map.entry((agt(i), oi.clone())) {
                        Vacant(e) => {
                            let obs_i = obs(r.obs[i].len());
                            r.obs[i].push(Vec::new());
                            e.insert(obs_i);
                            obs_map_reverse[i].push(oi);
                            obs_i
                        },
                        Occupied(e) => {
                            *e.get()
                        }
                    };
                    let obs_set = &mut r.obs[i][obs_i.index()];

                    obs_.push(obs_i);
                    obs_offset.push(obs_set.len());

                    obs_set.push(n);
                }

                r.loc.push(LocData {
                    successors: Vec::new(),
                    predecessors: Vec::new(),
                    is_winning: g.is_winning(&l),
                    obs: (*obs_).try_into().unwrap(),
                    obs_offset: (*obs_offset).try_into().unwrap()
                });

                queue.push_back(l.clone());

                loc_map.insert(l.clone(), n);
                loc_map_reverse.push(l);

                n
            }
        }
    }

    visit!(g.l0());

    let mut i: u32 = 0;
    while let Some(l) = queue.pop_front() {
        let n = loc(i);

        // print(|f| g.fmt_loc(f, &l));

        g.succ(&l, |a, l2| {
            let n2 = if let Some(&n2) = loc_map.get(&l2) {
                n2
            } else {
                visit!(l2)
            };
            if !r.successors(n).iter().any(|&(a_, n_)| (a, n2) == (a_, n_)) {
                r[n].successors.push((a, n2));
                r[n2].predecessors.push((a, n));
            }
        });

        i += 1;
    }

    for l in 0..r.n_loc() {
        // sort by action
        r.loc[l].successors.sort_unstable();
        r.loc[l].predecessors.sort_unstable();
    }

    let o = Rc::new(OriginImpl::new(
        g,
        loc_map,
        obs_map,
        loc_map_reverse,
        obs_map_reverse
    ));

    if keep_origin {
        r.origin = Some(o.clone() as Rc<dyn Origin>);
    }

    ConstructedGame { game: Rc::new(r), origin: o }
}
