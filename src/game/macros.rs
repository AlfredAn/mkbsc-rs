macro_rules! derive_ii {
    ($name:ty, $($tp:tt)*) => {
        impl<$($tp)*> IIGame for $name {
            type ObsId = Self::NodeId;
            fn observe(&self, l: Self::NodeId) -> Self::ObsId { l }
        }
    };
}

macro_rules! derive_ma {
    ($name:ty, $($tp:tt)*) => {
        impl<$($tp)*> MAGame for $name {
            type AgentId = crate::game::dgame::index::ZeroIndex;
            type AgentActId = Self::ActionId;
            fn n_agents(&self) -> usize { 1 }
            fn act_i(&self, act: Self::ActionId, _: Self::AgentId) -> Self::AgentActId { act }
        }
    }
}

macro_rules! derive_magiian {
    ($name:ty, $($tp:tt)*) => {
        impl<$($tp)*> MAGIIAN for $name {
            type AgentObsId = Self::ObsId;
            fn obs_i(&self, obs: Self::ObsId, _: Self::AgentId) -> Self::AgentObsId { obs }
        }
    }
}

pub(crate) use derive_ii;
pub(crate) use derive_ma;
pub(crate) use derive_magiian;
