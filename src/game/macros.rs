macro_rules! derive_ii {
    ($na:expr) => {
        type Obs = Self::Loc;
        fn observe(&self, l: &Self::Loc) -> [Self::Obs; $na] { array_init::array_init(|_| l.clone()) }
    };
}

macro_rules! derive_ma {
    ($l:lifetime) => {
        type Agent = crate::game::dgame::index::ZeroIndex;
        fn actions_i<'z>(&'z self, _: Self::Agent) -> crate::util::Itr<'z, Self::Act>
        where
            $l: 'z
        {
            Box::new(self.actions().map(|[a]| a))
        }
    }
}

macro_rules! derive_magiian {
    () => {}
}

/*macro_rules! post_set {
    ($na:expr) => {
        type PostSet<'b, I> where Self: 'b, I: 'b = impl Iterator<Item=Self::Loc> + Clone + 'b;
        fn post_set<'b, I>(&'b self, ns: I, a: [Self::Act; $na]) -> Self::PostSet<'b, I>
        where
            I: IntoIterator<Item=&'b Self::Loc> + 'b,
            I::IntoIter: Clone
        {
            crate::game::post_set_default(self, ns, a)
        }
    };
}*/

pub(crate) use derive_ii;
pub(crate) use derive_ma;
pub(crate) use derive_magiian;
//pub(crate) use post_set;
