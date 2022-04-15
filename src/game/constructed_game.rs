use crate::*;
use crate::hash_map::Entry::*;

#[derive(Clone, new)]
pub struct ConstructedGame<G, const N: usize>
where
    G: AbstractGame<N> + ?Sized
{
    pub origin: Rc<G>,
    pub game: Rc<Game<G::Data, N>>
}

impl<G, const N: usize> Debug for ConstructedGame<G, N>
where
    G: AbstractGame<N> + ?Sized,
    G::Data: Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConstructedGame")
            .field("origin", &Box::new(format!("{}", std::any::type_name::<G>())))
            .field("game", &self.game)
            .finish()
    }
}

impl<G, const N: usize> From<ConstructedGame<G, N>> for Rc<G>
where
    G: AbstractGame<N> + ?Sized
{
    fn from(g: ConstructedGame<G, N>) -> Self { g.origin }
}

impl<G, const N: usize> From<ConstructedGame<G, N>> for Rc<Game<G::Data, N>>
where
    G: AbstractGame<N> + ?Sized
{
    fn from(g: ConstructedGame<G, N>) -> Self { g.game }
}

impl<G, const N: usize> From<&ConstructedGame<G, N>> for Rc<G>
where
    G: AbstractGame<N> + ?Sized
{
    fn from(g: &ConstructedGame<G, N>) -> Self { g.origin.clone() }
}

impl<G, const N: usize> From<&ConstructedGame<G, N>> for Rc<Game<G::Data, N>>
where
    G: AbstractGame<N> + ?Sized
{
    fn from(g: &ConstructedGame<G, N>) -> Self { g.game.clone() }
}

impl<G, const N: usize> Deref for ConstructedGame<G, N>
where
    G: AbstractGame<N> + ?Sized
{
    type Target = Rc<Game<G::Data, N>>;
    fn deref(&self) -> &Self::Target {
        &self.game
    }
}

pub fn build_game<G, const N: usize>(g: Rc<G>) -> ConstructedGame<G, N>
where
    G: AbstractGame<N> + ?Sized
{
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
                    successors: Vec::new(),
                    predecessors: Vec::new(),
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
            if !r.successors(n).iter().any(|&(a_, n_)| (a, n2) == (a_, n_)) {
                r[n].successors.push((a, n2));
                r[n2].predecessors.push((a, n));
            }
        });

        i += 1;
    }

    // sort by action
    for l in 0..r.n_loc() {
        r.loc[l].successors.sort_unstable();
        r.loc[l].predecessors.sort_unstable();
    }

    ConstructedGame::new(g, Rc::new(r))
}
