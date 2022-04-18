use crate::*;

#[derive(Debug, new, Clone)]
pub struct AllStrategies<T: Clone, const N: usize> {
    parts: [AllStrategies1<KBSCData<T>>; N],
    gk: ConstructedGame<MKBSC<T, N>, N>
}

#[derive(new, Clone)]
pub struct MKBSCStratProfile<T: Clone, S: Strategy<KBSCData<T>>, const N: usize> {
    s: [S; N],
    gk: ConstructedGame<MKBSC<T, N>, N>
}

impl<T: Clone, S: Strategy<KBSCData<T>>, const N: usize> Debug for MKBSCStratProfile<T, S, N>
where S: Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_list(f, 0..N, |f, agt|
            write!(f, "{:?}", self.s[agt])
        )
    }
}

// implement inverse of this
impl<T: Clone, S: Strategy<KBSCData<T>>, const N: usize> StrategyProfile<MKBSCData<T, N>, N> for MKBSCStratProfile<T, S, N> {
    type M = S::M;

    fn call(&self, agt: Agt, o_gk: Obs<MKBSCData<T, N>>, mem: &Self::M) -> Option<(Act, Self::M)> {
        let (gk, gki) = (&self.gk.game, &self.gk.origin.gki[agt]);
        let l_gk = gk.obs_set(agt, o_gk)[0];
        let l_gki = gk.data(l_gk).0[agt].0;
        let [o_gki] = gki.observe(l_gki);

        //println!("        (o_gk, l_gk, l_gki, o_gki)={:?}", (o_gk, l_gk, l_gki, o_gki));

        self.s[agt].call(o_gki, mem)
    }

    fn init(&self) -> [Self::M; N] {
        array_init(|i|
            self.s[i].init()
        )
    }
}

impl<T: Clone, const N: usize> AllStrategies<T, N> {
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
    -> [AbstractTransducer<T, MlessStrat<KBSCData<T>, Vec<Option<Act>>>>; N] {
        array_init(|i|
            transducer(self.gk.origin.gki[i].clone(), self.parts[i].get())
        )
    }
    
    pub fn get(&self) -> MKBSCStratProfile<T, MlessStrat<KBSCData<T>, Vec<Option<Act>>>, N> {
        MKBSCStratProfile::new(
            array_init(|i| self.parts[i].get()),
            self.gk.clone()
        )
    }

    pub fn get_ref(&self) -> MKBSCStratProfile<T, MlessStrat<KBSCData<T>, &Vec<Option<Act>>>, N> {
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

pub fn all_strategies<T: Clone, const N: usize>(mkbsc: &ConstructedGame<MKBSC<T, N>, N>) -> AllStrategies<T, N> {
    AllStrategies::new(
        array_init(|i|
            all_strategies1(&mkbsc.origin.gki[i])
        ),
        mkbsc.clone()
    )
}

pub trait Strategy<T> {
    type M: Clone + Eq + Hash;
    fn call(&self, obs: Obs<T>, mem: &Self::M) -> Option<(Act, Self::M)>;
    fn init(&self) -> Self::M;
}

pub trait MemorylessStrategy<T>: Strategy<T, M=()> {
    fn call_ml(&self, obs: Obs<T>) -> Option<Act> {
        self.call(obs, &()).map(|(a, _)| a)
    }
}

impl<T, S> MemorylessStrategy<T> for S
where S: Strategy<T, M=()> {}

pub trait StrategyProfile<T, const N: usize> {
    type M: Clone + Eq + Hash;
    fn call(&self, agt: Agt, obs: Obs<T>, mem: &Self::M) -> Option<(Act, Self::M)>;
    fn init(&self) -> [Self::M; N];

    fn get(&self, index: Agt) -> StratProfileProject<T, Self, N> where Self: Sized {
        StratProfileProject::new(self, index)
    }
}

impl<T, S: Strategy<T>, const N: usize> StrategyProfile<T, N> for [S; N] {
    type M = S::M;

    fn call(&self, agt: Agt, obs: Obs<T>, mem: &Self::M) -> Option<(Act, Self::M)> {
        self[agt].call(obs, mem)
    }

    fn init(&self) -> [Self::M; N] {
        array_init(|i|
            self[i].init()
        )
    }
}

#[derive(new, Debug, Clone)]
pub struct StratProfileProject<'a, T, S: StrategyProfile<T, N>, const N: usize>(&'a S, Agt, PhantomData<T>);

impl<'a, T, S: StrategyProfile<T, N>, const N: usize> Strategy<T> for StratProfileProject<'a, T, S, N> {
    type M = S::M;

    fn call(&self, obs: Obs<T>, mem: &Self::M) -> Option<(Act, Self::M)> {
        self.0.call(self.1, obs, mem)
    }

    fn init(&self) -> Self::M {
        self.0.init()[self.1].clone()
    }
}

/*pub trait MemorylessStrategyProfile<T, const N: usize>: StrategyProfile<T, N, M=()> {
    fn call_ml(&self, agt: Agt, obs: Obs<T>) -> Option<Act> {
        self.call(agt, obs, &())
    }
}*/
