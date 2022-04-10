pub mod project;
pub mod kbsc;
pub mod dkbsc;
pub mod mkbsc;
pub mod strat_synth;

use std::collections::HashSet;
use crate::game::VisitSet;
use crate::*;
pub use project::*;
pub use kbsc::*;
pub use mkbsc::*;
pub use strat_synth::*;

/// Perform depth first search on `g`, calling the function `node`
/// for each location, and `edge` for each transition.
/// 
/// `node` is guaranteed to only be called once for each location,
/// while `edge` can be called multiple times for one transition if `g.post()`
/// returns it multiple times.
/// 
/// `node` will always be called on a location before it is used in a call to `edge`.
pub fn explore<G: Game<N> + ?Sized, const N: usize>(
    g: &G,
    mut node: impl FnMut(&G::Loc),
    mut edge: impl FnMut(&G::Loc, [G::Act; N], &G::Loc)
) {
    let mut stack = Vec::new();
    let mut visited = HashSet::new();

    let l0 = g.l0();
    node(l0);
    visited.insert(l0.clone());
    stack.push(l0.clone());

    while let Some(l) = stack.pop() {
        for a in g.actions() {
            for l2 in g.post(&l, a) {
                let is_visited = visited.contains(&l2);

                if !is_visited {
                    node(&l2);
                }

                edge(&l, a, &l2);

                if !is_visited {
                    visited.insert(l2.clone());
                    stack.push(l2);
                }
            }
        }
    }
}

pub fn explore1<G: Game1>(
    g: &G,
    node: impl FnMut(&G::Loc),
    mut edge: impl FnMut(&G::Loc, G::Act, &G::Loc)
) {
    explore(g, node, |l, [a], l2| edge(l, a, l2))
}

pub enum SimAction<A, M, E> {
    Visit(A, M),
    Skip,
    Stop(E)
}

pub fn simulate<'a, G, M, E, const N: usize>(
    g: &G,
    init: M,
    mut f: impl FnMut(&G::Loc, &M, bool)
        -> SimAction<[G::Act; N], M, E>
) -> Result<(), E>
where
    G: Game<N> + HasVisitSet<N>,
    M: Clone
{
    let mut stack = vec![(g.l0().clone(), init)];
    let mut visited = g.visit_set();

    while let Some((l, mem)) = stack.pop() {
        let is_visited = visited.contains(&l);
        match f(&l, &mem, is_visited) {
            SimAction::Visit(a, mem) => {
                for l2 in g.post(&l, a) {
                    stack.push((l2, mem.clone()));
                }
            },
            SimAction::Skip => (),
            SimAction::Stop(result) => {
                return Err(result);
            }
        }
        visited.insert(l);
    }

    Ok(())
}
