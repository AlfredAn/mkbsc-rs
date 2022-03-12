use petgraph::visit::{Dfs, Visitable, IntoNeighbors, GraphBase, VisitMap};

use crate::{game::Game};

pub mod play;

pub fn for_each_location<'a, G, F>(g: &'a G, mut f: F)
where
    &'a G: Game + Visitable + IntoNeighbors,
    <&'a G as Visitable>::Map: VisitMap<<&'a G as GraphBase>::NodeId>,
    F: FnMut(<&'a G as GraphBase>::NodeId)
{
    let mut dfs = Dfs::new(g, g.l0());

    while let Some(n) = dfs.next(g) {
        f(n);
    }
}

pub fn count_locations<'a, G>(g: &'a G) -> usize
where
    &'a G: Game + Visitable + IntoNeighbors,
    <&'a G as Visitable>::Map: VisitMap<<&'a G as GraphBase>::NodeId>
{
    let mut i = 0;
    for_each_location(g, |_| i += 1);
    i
}
