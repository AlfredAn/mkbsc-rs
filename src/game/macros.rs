macro_rules! derive_ii {
    ($na:expr) => {
        type Obs = Self::Loc;
        fn observe(&self, l: &Self::Loc) -> [Self::Obs; $na] { [l.clone(); $na] }
    };
}

macro_rules! derive_ma {
    ($l:lifetime) => {
        type Agent = crate::game::dgame::index::ZeroIndex;
        type ActionsI = impl Iterator<Item=Self::Act>;
        fn actions_i(&self, _: Self::Agent) -> Self::ActionsI { self.actions().map(|[a]| a) }
    }
}

macro_rules! derive_magiian {
    () => {}
}

macro_rules! post_set {
    ($na:expr, $l:lifetime) => {
        /*type PostSet<'b> where Self: 'b = impl Iterator<Item=Self::Loc>;
        fn post_set<'b>(&'b self, ns: Vec<Self::Loc>, a: [Self::Act; $na]) -> Self::PostSet<'b> {
            use itertools::Itertools;
            ns.into_iter()
                .map(move |n| self.post(&n, a))
                .flatten()
                .unique()
        }*/
    };
}

pub(crate) use derive_ii;
pub(crate) use derive_ma;
pub(crate) use derive_magiian;
//pub(crate) use post_set;
