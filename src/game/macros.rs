macro_rules! derive_ii {
    () => {
        type Obs = Self::Loc;
        fn observe(&self, l: Self::Loc) -> Self::Obs { l }
    };
}

macro_rules! derive_ma {
    ($l:lifetime) => {
        type Agent = crate::game::dgame::index::ZeroIndex;
        type ActionsI = impl Iterator<Item=Self::Act>;
        fn actions_i(&$l self, _: Self::Agent) -> Self::ActionsI { self.actions().map(|[a]| a) }
    }
}

macro_rules! derive_magiian {
    () => {
        type AgentObs = Self::Obs;
        fn obs_i(&self, obs: Self::Obs, _: Self::Agent) -> Self::AgentObs { obs }
    }
}

pub(crate) use derive_ii;
pub(crate) use derive_ma;
pub(crate) use derive_magiian;
