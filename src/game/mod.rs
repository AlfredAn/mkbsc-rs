use petgraph::{visit::{GraphBase}};

pub mod dgame;

pub trait Game: GraphBase {
    type ActionId: Copy + PartialEq;
    fn l0(&self) -> Self::NodeId;
}

pub trait IIGame: Game {
    type ObsId: Copy + PartialEq;
    fn observe(&self, l: Self::NodeId) -> Self::ObsId;
}

impl<G: Game> Game for &G {
    type ActionId = G::ActionId;
    fn l0(&self) -> Self::NodeId { (*self).l0() }
}

impl<G: IIGame> IIGame for &G {
    type ObsId = G::ObsId;
    fn observe(&self, l: Self::NodeId) -> Self::ObsId { (*self).observe(l) }
}
