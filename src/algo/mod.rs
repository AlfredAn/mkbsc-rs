use petgraph::visit::{Dfs, Visitable, IntoNeighbors, GraphBase, VisitMap};

use crate::{game::Game};

pub fn count_locations<'a, G>(g: &'a G) -> usize
where
    &'a G: Game + Visitable + IntoNeighbors,
    <&'a G as Visitable>::Map: VisitMap<<&'a G as GraphBase>::NodeId>
{
    let l0 = g.l0();
    let mut dfs = Dfs::new(g, l0);

    let mut i = 0;
    while let Some(_) = dfs.next(g) {
        i += 1;
    }

    i
}
