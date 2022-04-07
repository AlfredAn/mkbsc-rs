pub mod project;
pub mod kbsc;
pub mod mkbsc;
pub mod strat_synth;

use crate::game::VisitSet;
use crate::*;
pub use project::*;
pub use kbsc::*;
pub use mkbsc::*;
pub use strat_synth::*;

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
    G: Game<'a, N> + HasVisitSet<'a, N>,
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
