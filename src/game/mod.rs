use petgraph::{visit::{GraphBase}};

pub mod dgame;

pub trait Game: GraphBase {
    type ActionId: Copy + PartialEq;
    type ObsId: Copy + PartialEq;
    
    fn l0(&self) -> Self::NodeId;
    fn observe(&self, l: Self::NodeId) -> Self::ObsId;
}

impl<G: Game> Game for &G {
    type ActionId = G::ActionId;
    type ObsId = G::ObsId;

    fn l0(&self) -> Self::NodeId { (*self).l0() }
    fn observe(&self, l: Self::NodeId) -> Self::ObsId { (*self).observe(l) }
}
