use crate::*;

#[derive(Debug, new, Clone)]
pub struct AllStrategies<const N: usize> {
    parts: [AllStrategies1; N],
    gk: ConstructedGame<MKBSC<N>, N>
}

impl<const N: usize> AllStrategies<N> {
    pub fn advance(&mut self) -> bool {
        for p in &mut self.parts {
            if p.advance() {
                return true;
            } else {
                p.reset();
            }
        }

        false
    }

    pub fn transducers(&self)
    -> [AbstractTransducer<MlessStrat<Vec<Option<Act>>>>; N] {
        array_init(|i|
            transducer(self.gk.origin().gki[i].clone(), self.parts[i].get())
        )
    }
    
    pub fn get(&self) -> MKBSCStratProfile<[MlessStrat<Vec<Option<Act>>>; N], N> {
        MKBSCStratProfile::new(
            array_init(|i| self.parts[i].get()),
            self.gk.clone()
        )
    }

    pub fn get_ref(&self) -> MKBSCStratProfile<[MlessStrat<&Vec<Option<Act>>>; N], N> {
        MKBSCStratProfile::new(
            array_init(|i| self.parts[i].get_ref()),
            self.gk.clone()
        )
    }

    pub fn get_raw(&self) -> [&Vec<Option<Act>>; N] {
        array_init(|i| self.parts[i].get_raw())
    }

    pub fn reset(&mut self) {
        for p in &mut self.parts {
            p.reset();
        }
    }
}

pub fn all_strategies<const N: usize>(mkbsc: ConstructedGame<MKBSC<N>, N>) -> AllStrategies<N> {
    AllStrategies::new(
        array_init(|i|
            all_strategies1(&mkbsc.origin().gki[i])
        ),
        mkbsc
    )
}

pub trait Strategy {
    type M: Clone + Eq + Hash;
    fn call(&self, obs: Obs, mem: &Self::M) -> Option<(Act, Self::M)>;
    fn init(&self) -> Self::M;
}

pub trait MemorylessStrategy: Strategy<M=()> {
    fn call_ml(&self, obs: Obs) -> Option<Act> {
        self.call(obs, &()).map(|(a, _)| a)
    }
}

impl<S> MemorylessStrategy for S
where S: Strategy<M=()> {}

pub trait StrategyProfile<const N: usize> {
    type M: Clone + Eq + Hash;
    fn call(&self, agt: Agt, obs: Obs, mem: &Self::M) -> Option<(Act, Self::M)>;
    fn init(&self) -> [Self::M; N];

    fn project(&self, index: Agt) -> StratProfileProject<Self, N> where Self: Sized {
        StratProfileProject::new(self, index)
    }
}

impl<S: Strategy, const N: usize> StrategyProfile<N> for [S; N] {
    type M = S::M;

    fn call(&self, agt: Agt, obs: Obs, mem: &Self::M) -> Option<(Act, Self::M)> {
        self[agt].call(obs, mem)
    }

    fn init(&self) -> [Self::M; N] {
        array_init(|i|
            self[i].init()
        )
    }
}

#[derive(new, Debug, Clone)]
pub struct StratProfileProject<'a, S: StrategyProfile<N>, const N: usize>(&'a S, Agt);

impl<'a, S: StrategyProfile<N>, const N: usize> Strategy for StratProfileProject<'a,  S, N> {
    type M = S::M;

    fn call(&self, obs: Obs, mem: &Self::M) -> Option<(Act, Self::M)> {
        self.0.call(self.1, obs, mem)
    }

    fn init(&self) -> Self::M {
        self.0.init()[self.1].clone()
    }
}

#[derive(new, Clone)]
pub struct MKBSCStratProfile<S: StrategyProfile<N>, const N: usize> {
    s: S,
    gk: ConstructedGame<MKBSC<N>, N>
}

impl<S: StrategyProfile<N>, const N: usize> Debug for MKBSCStratProfile<S, N>
where S: Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.s)
    }
}

impl<S: StrategyProfile<N>, const N: usize> StrategyProfile<N> for MKBSCStratProfile<S, N> {
    type M = S::M;

    fn call(&self, agt: Agt, o_gk: Obs, mem: &Self::M) -> Option<(Act, Self::M)> {
        let (gk, gki) = (&self.gk, &self.gk.origin().gki[agt]);
        
        let l_gk = gk.obs_set(agt, o_gk)[0];
        let l_gki = gk.origin_loc(l_gk)[agt];
        let [o_gki] = gki.observe(l_gki);

        self.s.call(agt, o_gki, mem)
    }

    fn init(&self) -> [Self::M; N] {
        self.s.init()
    }
}

#[derive(new, Clone)]
pub struct KBSCStratProfile<S: StrategyProfile<N>, const N: usize> {
    s: S,
    gk: ConstructedGame<MKBSC<N>, N>
}

impl<S: StrategyProfile<N>, const N: usize> Debug for KBSCStratProfile<S, N>
where S: Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.s)
    }
}

impl<S: StrategyProfile<N>, const N: usize> StrategyProfile<N> for KBSCStratProfile<S, N> {
    type M = S::M;

    fn call(&self, agt: Agt, o_gki: Obs, mem: &Self::M) -> Option<(Act, Self::M)> {
        let (gk, gki) = (&self.gk, &self.gk.origin().gki[agt]);
        let l_gki = gki.to_unique_loc(o_gki, 0).unwrap();
        let o_gk = gk.obs(&(agt, l_gki));

        self.s.call(agt, o_gk, mem)
    }

    fn init(&self) -> [Self::M; N] {
        self.s.init()
    }
}
