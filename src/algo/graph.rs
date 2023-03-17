use crate::*;

pub trait Graph {
    type EdgeData: Debug;

    fn successors(&self, from: Loc) -> Box<dyn Iterator<Item=(Self::EdgeData, Loc)> + '_>;
    fn predecessors(&self, to: Loc) -> Box<dyn Iterator<Item=(Self::EdgeData, Loc)> + '_>;

    fn edges_between(&self, from: Loc, to: Loc) -> Box<dyn Iterator<Item=Self::EdgeData> + '_> {
        Box::new(
            self.successors(from)
                .filter_map(move |(edge_data, to2)|
                    if to2 == to { Some(edge_data) } else { None }
                )
        )
    }

    fn size(&self) -> usize;
}

impl<const N: usize> Graph for Game<N> {
    type EdgeData = [Act; N];

    fn successors(&self, from: Loc) -> Box<dyn Iterator<Item=(Self::EdgeData, Loc)> + '_> {
        Box::new(self.successors(from).iter().copied())
    }

    fn predecessors(&self, to: Loc) -> Box<dyn Iterator<Item=(Self::EdgeData, Loc)> + '_> {
        Box::new(self.predecessors(to).iter().copied())
    }

    fn size(&self) -> usize {
        self.n_loc()
    }
}
