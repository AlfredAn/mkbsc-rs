
use std::num::NonZeroU32;
use MapEntry::*;
use crate::*;
use crate::game::{*, dgame::from_game::*};
use std::collections::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MapEntry<A> {
    Absent,
    Winning,
    Action(NonZeroU32, A)
}

/*trait FindStrat<'a>: Game<'a, 1> {
    fn find_memoryless_strategy(&'a self) -> Vec<MapEntry<ActionIndex>>;
}

impl<'a, G: Game<'a, 1>> FindStrat<'a> for G {
    default fn find_memoryless_strategy(&'a self) -> Vec<MapEntry<ActionIndex>> {
        let dg = dgame(self);
        _find_memoryless_strategy(&dg)
    }
}

impl<'a> FindStrat<'a> for DGame<1> {
    fn find_memoryless_strategy(&'a self) -> Vec<MapEntry<ActionIndex>> {
        _find_memoryless_strategy(self)
    }
}*/

fn _find_memoryless_strategy(g: &DGame<1>) -> Vec<MapEntry<ActionIndex>>
{
    let mut w = vec![Absent; g.graph.node_count()];
    let mut w_list = Vec::new();
    explore(g, |l|
        if w[l.index()] == Absent && g.is_winning(&l) {
            w_list.push(l);
            w[l.index()] = Winning;
        }
    );
    
    let mut buf = Vec::new();
    let mut depth: u32 = 1;
    loop {
        println!("depth={}", depth);
        for (l, a) in g.cpre(|i| w[i] != Absent, w_list.iter().copied()) {
            println!("  cpre: {:?}", (l.index(), a.index()));
            buf.push((l, a, depth));
        }

        let mut inserted = false;
        for (l, a, depth) in buf.drain(..) {
            if w[l.index()] == Absent {
                println!("  insert: {:?}", l.index());
                w[l.index()] = Action(depth.try_into().unwrap(), a);
                w_list.push(l);
                inserted = true;
            }
        }
        if !inserted { break; }

        depth += 1;
    }

    w
}

pub fn explore<'a, G, F, const N: usize>(g: &G, mut f: F)
where
    G: Game<'a, N>,
    G::Loc: Ord,
    F: FnMut(G::Loc)
{
    let mut visited = BTreeSet::new();
    let mut stack = vec![g.l0().clone()];

    while let Some(l) = stack.pop() {
        if !visited.contains(&l) {
            visited.insert(l.clone());

            for a in g.actions() {
                for l2 in g.post(&l, a) {
                    stack.push(l2);
                }
            }

            f(l);
        }
        
    }
}
