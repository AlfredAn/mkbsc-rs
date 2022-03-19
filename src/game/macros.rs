macro_rules! derive_ii {
    () => {
        type Obs = Self::Loc;
        fn observe(&self, l: Self::Loc) -> Self::Obs { l }
    };
}

macro_rules! derive_ma {
    () => {
        type Agent = crate::game::dgame::index::ZeroIndex;
        type AgentAct = Self::Act;
        type ActionsI<'reallylongname> where Self: 'reallylongname = impl Iterator<Item=Self::AgentAct>;
        fn n_agents(&self) -> usize { 1 }
        fn act_i(&self, act: Self::Act, _: Self::Agent) -> Self::AgentAct { act }
        fn actions_i(&self, _: Self::Agent) -> Self::ActionsI<'_> { self.actions() }
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
