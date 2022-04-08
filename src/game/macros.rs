macro_rules! derive_ii {
    ($na:expr) => {
        type Obs = Self::Loc;
        fn observe(&self, l: &Self::Loc) -> [Self::Obs; $na] { array_init::array_init(|_| l.clone()) }
    };
}

macro_rules! derive_ma {
    () => {
        type Agt = crate::game::dgame::index::ZeroIndex;
        fn actions_i<'z>(&'z self, _: Self::Agt) -> crate::util::Itr<'z, Self::Act> {
            Box::new(self.actions().map(|[a]| a))
        }
    }
}

macro_rules! derive_magiian {
    () => {}
}
