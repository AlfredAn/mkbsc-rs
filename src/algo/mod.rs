use crate::*;

pub mod strat_synth;

/// Perform depth first search on `g`, calling the function `node`
/// for each location, and `edge` for each transition.
/// 
/// `node` is guaranteed to only be called once for each location,
/// while `edge` can be called multiple times for one transition if `g.post()`
/// returns it multiple times.
/// 
/// `node` will always be called on a location before it is used in a call to `edge`.
pub fn explore<T, const N: usize>(
    g: &Game<T, N>,
    mut node: impl FnMut(Loc),
    mut edge: impl FnMut(Loc, [Act; N], Loc)
) {
    let mut stack = Vec::new();
    let mut visited = LocSet::new(g);

    let l0 = g.l0();
    node(l0);
    visited.insert(l0);
    stack.push(l0);

    while let Some(l) = stack.pop() {
        for a in g.action_profiles() {
            for l2 in g.post(l, a) {
                let is_visited = visited.contains(l2);

                if !is_visited {
                    node(l2);
                }

                edge(l, a, l2);

                if !is_visited {
                    visited.insert(l2);
                    stack.push(l2);
                }
            }
        }
    }
}

pub fn explore1<T>(
    g: &Game<T, 1>,
    node: impl FnMut(Loc),
    mut edge: impl FnMut(Loc, Act, Loc)
) {
    explore(g, node, |l, [a], l2| edge(l, a, l2))
}

pub enum SimAction<A, M, E> {
    Visit(A, M),
    Skip,
    Stop(E)
}

pub fn simulate<'a, T, M: Clone, E, const N: usize>(
    g: &Game<T, N>,
    init: M,
    mut f: impl FnMut(Loc, &M, bool)
        -> SimAction<[Act; N], M, E>
) -> Result<(), E> {
    let mut stack = vec![(g.l0(), init)];
    let mut visited = LocSet::new(g);

    while let Some((l, mem)) = stack.pop() {
        let is_visited = visited.contains(l);
        match f(l, &mem, is_visited) {
            SimAction::Visit(a, mem) => {
                for l2 in g.post(l, a) {
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
