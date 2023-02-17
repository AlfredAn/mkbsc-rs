use anyhow::Context;
use bit_set::BitSet;
use vec_map::VecMap;

use crate::*;

type Depth = u32;

const DEBUG: bool = false;
macro_rules! debug {
    ($s:expr) => {
        if DEBUG { println!("{:?}", $s) };
    };
    ($s:expr, $s2:expr) => {
        if DEBUG { println!("{}{:?}", $s, $s2) };
    };
}

struct State<'game, const N: usize> {
    g: [&'game Game<N>; 2],
    check_obs: bool,

    m: [VecMap<Loc>; 2],
    m_obs: [[VecMap<(Obs, Depth)>; N]; 2],

    in_set: [VecMap<Depth>; 2],
    out_set: [VecMap<Depth>; 2],

    visited_count: [usize; 2],

    buf: BitSet
}

macro_rules! ins {
    ($self:ident, $s:ident, $i:expr, $l:expr) => {{
        let (i, l) = ($i, $l);
        let depth = $self.m[0].len() as u32;
        debug!(format!("ins: i={}, l={}, d={}, s={:?}", i, l, depth, $self.$s[i]));
        if !$self.$s[i].contains_key(l.index()) {
            if !$self.visited(i, l) { $self.visited_count[i] += 1; }
            $self.$s[i].insert(l.index(), depth);
        }
    }}
}

macro_rules! rm {
    ($self:ident, $s:ident, $i:expr, $l:expr) => {{
        let (i, l) = ($i, $l);
        let depth = $self.m[0].len() as u32;
        debug!(format!("rm: i={}, l={}, d={}, s={:?}", i, l, depth, $self.$s[i]));
        if $self.$s[i][l.index()] == depth {
            $self.$s[i].remove(l.index());
            if !$self.visited(i, l) { $self.visited_count[i] -= 1; }
        }
    }}
}

impl<'g, const N: usize> State<'g, N> {
    fn new(g: [&'g Game<N>; 2], check_obs: bool) -> Self {
        let n = g[0].n_loc();
        Self {
            g,
            check_obs,

            m: [VecMap::with_capacity(n), VecMap::with_capacity(n)],
            m_obs: array_init(|_| array_init(|i|
                if check_obs {
                    VecMap::with_capacity(g[0].n_obs(agt(i)))
                } else {
                    VecMap::new()
                }
            )),

            in_set: [VecMap::with_capacity(n), VecMap::with_capacity(n)],
            out_set: [VecMap::with_capacity(n), VecMap::with_capacity(n)],

            visited_count: [0, 0],

            buf: BitSet::new()
        }
    }

    fn visited(&self, i: usize, l: Loc) -> bool {
        self.in_set[i].contains_key(l.index())
         || self.out_set[i].contains_key(l.index())
    }

    fn m(&self, i: usize, l: Loc) -> Option<Loc> {
        self.m[i].get(l.index()).copied()
    }

    fn tin(&self, i: usize, l: Loc) -> bool {
        self.in_set[i].contains_key(l.index())
            && !self.m[i].contains_key(l.index())
    }

    fn tout(&self, i: usize, l: Loc) -> bool {
        self.out_set[i].contains_key(l.index())
            && !self.m[i].contains_key(l.index())
    }

    fn tin_len(&self, i: usize) -> usize {
        let (ml, il) = (self.m[i].len(), self.in_set[i].len());
        il.checked_sub(ml)
            .with_context(|| format!("m larger than in_set ({} > {})", ml, il))
            .unwrap()
    }

    fn tout_len(&self, i: usize) -> usize {
        let (ml, ol) = (self.m[i].len(), self.out_set[i].len());
        ol.checked_sub(ml)
            .with_context(|| format!("m larger than out_set ({} > {})", ml, ol))
            .unwrap()
    }
    
    fn out_insert(&mut self, i: usize, l: Loc, depth: Depth) {
        if !self.out_set[i].contains_key(l.index()) {
            if !self.visited(i, l) { self.visited_count[i] += 1; }
            self.out_set[i].insert(l.index(), depth);
        }
    }

    fn add_mapping(&mut self, l: [Loc; 2], obs: [[Obs; N]; 2]) {
        debug!(format!("add_mapping: {}<->{}", l[0], l[1]));

        for i in [0, 1] {
            assert!(!self.m[i].contains_key(l[i].index()));

            self.m[i].insert(l[i].index(), l[1-i]);
            ins!(self, out_set, i, l[i]);
            ins!(self, in_set, i, l[i]);

            for (_, l2) in self.g[i].successors(l[i]) {
                ins!(self, out_set, i, *l2);
            }
            for (_, l2) in self.g[i].predecessors(l[i]) {
                ins!(self, in_set, i, *l2);
            }

            if self.check_obs {
                let depth = self.m[0].len() as u32;
                for agt in 0..N {
                    self.m_obs[i][agt].insert(obs[i][agt].index(), (obs[1-i][agt], depth));
                }
            }
        }
        debug!("m", self.m);
        debug!("out_set", self.out_set);
        debug!("in_set", self.in_set);
    }

    fn remove_mapping(&mut self, l: [Loc; 2], obs: [[Obs; N]; 2]) {
        debug!("remove_mapping");
        for i in [0, 1] {
            assert!(self.m[i].contains_key(l[i].index()));

            self.m[i].remove(l[i].index());
            rm!(self, out_set, i, l[i]);
            rm!(self, in_set, i, l[i]);
            
            for (_, l2) in self.g[i].successors(l[i]) {
                rm!(self, out_set, i, *l2);
            }
            for (_, l2) in self.g[i].predecessors(l[i]) {
                rm!(self, in_set, i, *l2);
            }

            if self.check_obs {
                let depth = self.m[0].len() as u32;
                for agt in 0..N {
                    if self.m_obs[i][agt][obs[i][agt].index()].1 == depth {
                        self.m_obs[i][agt].remove(obs[i][agt].index());
                    }
                }
            }
        }
    }

    fn match_neighbors(&mut self, n: [&[([Act; N], Loc)]; 2]) -> bool {
        if n[0].len() != n[1].len() { return false; }

        self.buf.reserve_len(n[0].len());
        self.buf.clear();

        // match each element in n[0] with one in n[1]
        // can be optimized by storing edges in a set (l, a, l2)
        for &(a, l0) in n[0] {
            if let Some(l1) = self.m(0, l0) {
                if let Some((i, _)) = n[1].iter().enumerate().find(
                    |&(i, &e)| e == (a, l1) && !self.buf.contains(i)
                ) {
                    self.buf.insert(i);
                } else {
                    return false;
                }
            }
        }

        true
    }

    fn is_feasible(&mut self, l: [Loc; 2]) -> bool {
        if self.g[0].is_winning(l[0]) != self.g[1].is_winning(l[1]) { return false; }

        let (s0, s1) = (self.g[0].successors(l[0]), self.g[1].successors(l[1]));
        let (p0, p1) = (self.g[0].predecessors(l[0]), self.g[1].predecessors(l[1]));
        
        self.match_neighbors([s0, s1]) && self.match_neighbors([p0, p1])
    }

    fn is_obs_feasible(&mut self, obs: [[Obs; N]; 2]) -> bool {
        if !self.check_obs { return true; }

        (0..N).all(|ag| {
            let agt = agt(ag);
            [0, 1].map(|j|
                self.g[j].obs_set(agt, obs[j][ag]).len()
            ).iter().all_equal()
             && match [0, 1].map(|i|
                self.m_obs[i][ag].get(obs[i][ag].index())
            ) {
                [Some(o1), Some(o0)] => {
                    o0.0 == obs[0][ag] && o1.0 == obs[1][ag]
                },
                [None, None] => {
                    // could be made to prune the search tree even more here
                    // by doing something similar to match_neighbors
                    true
                },
                _ => false
            }
        })
    }
}

pub fn is_isomorphic<const N: usize>(g0: &Game<N>, g1: &Game<N>, check_obs: bool) -> bool {
    debug!("is_isomorphic");

    let g = [g0, g1];
    if g[0].n_loc() != g[1].n_loc()
    || (check_obs && (0..N).any(|i| g[0].n_obs(agt(i)) != g[1].n_obs(agt(i)))) {
        return false;
    }
    
    let mut state = State::new(g, check_obs);

    let l = g.map(|g| g.l0());
    let obs = g.map(|g| g.observe(g.l0()));

    if !state.is_feasible(l) || !state.is_obs_feasible(obs) {
        return false;
    }

    state.add_mapping(l, obs);

    let result = iso(&mut state);
    result
}

fn iso<const N: usize>(state: &mut State<N>) -> bool {
    debug!("iso");

    debug!("iso: checking set sizes");
    if state.tin_len(0) != state.tin_len(1)
    || state.tout_len(0) != state.tout_len(1)
    || state.visited_count[0] != state.visited_count[1] {
        return false;
    }

    let n = state.g[0].n_loc();

    assert_eq!(state.m[0].len(), state.m[1].len());

    debug!("iso: checking completion");
    if state.m[0].len() == n {
        return true;
    }

    macro_rules! expand {
        ($f:expr) => {
            for l1 in (0..n).map(|l| loc(l)) {
                if !($f)(1, l1) { continue; }

                for l0 in (0..n).map(|l| loc(l)) {
                    let l = [l0, l1];
                    if ($f)(0, l0) && state.is_feasible(l) {
                        let obs: [_; 2] = array_init(|i|
                            state.g[i].observe(l[i])
                        );
                        if state.is_obs_feasible(obs) {
                            state.add_mapping(l, obs);
                            if iso(state) {
                                return true;
                            }
                            state.remove_mapping(l, obs);
                        }
                    }
                }

                break;
            }
        }
    }

    let (o0, o1, i0, i1) = (
        state.tout_len(0) > 0,
        state.tout_len(1) > 0,
        state.tin_len(0) > 0,
        state.tin_len(1) > 0
    );

    debug!("iso: finding set P");
    match ((o0, o1), (i0, i1)) {
        ((true, true), (false, false)) | ((true, true), (true, true)) => {
            debug!("iso: expanding case 1");
            expand!(|i, l| state.tout(i, l));
        },
        ((false, false), (true, true)) => {
            debug!("iso: expanding case 2");
            expand!(|i, l| state.tin(i, l));
        },
        ((false, false), (false, false)) => {
            debug!("iso: expanding case 3");
            expand!(|i, l| state.m(i, l).is_none());
        },
        _ => { debug!("iso: not expanding"); }
    }

    debug!("iso: backtracking");
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cup_game() {
        let g = include_game!("../../games/cup_game", 2)
            .build().game;
        let mut stack = MKBSCStack::new(g);
        for _ in 0..4 {
            stack.push();
        }

        for i in 0..=4 {
            for j in 0..=4 {
                debug!("running test", (i, j));

                let (g0, g1) = (stack.get(i).game(), stack.get(j).game());
                debug!("---g0---", g0);
                debug!("---g1---", g1);

                assert_eq!(
                    is_isomorphic(g0, g1, false),
                    i == j || min(i, j) >= 1
                );
                assert_eq!(
                    is_isomorphic(g0, g1, true),
                    i == j || min(i, j) >= 2
                );
            }
        }
    }

    #[test]
    fn test_obs() {
        let g = [
            include_game!("../../games/test/test_obs_1.game", 1),
            include_game!("../../games/test/test_obs_2.game", 1),
        ].map(|g| g.build().game);

        assert!( is_isomorphic(&g[0], &g[0], true));
        assert!( is_isomorphic(&g[1], &g[1], true));
        assert!(!is_isomorphic(&g[0], &g[1], true));
        assert!(!is_isomorphic(&g[1], &g[0], true));

        assert!( is_isomorphic(&g[0], &g[0], false));
        assert!( is_isomorphic(&g[1], &g[1], false));
        assert!( is_isomorphic(&g[0], &g[1], false));
        assert!( is_isomorphic(&g[1], &g[0], false));
    }

    #[test]
    fn test_key_not_present_1() {
        for i in 0..50 {
            let g = [
                include_game!("../../games/test/test_iso_1", 2),
                include_game!("../../games/test/test_iso_2", 2),
            ].map(|g| g.build().game);

            println!("test #{}", i);
            println!("[{},\n{}]", g[0], g[1]);

            assert!(is_isomorphic(&g[0], &g[1], false));
        }
    }

    #[test]
    fn test_key_not_present_2() {
        for i in 0..50 {
            let g = [
                include_game!("../../games/test/test_iso_3", 2),
                include_game!("../../games/test/test_iso_4", 2),
            ].map(|g| g.build().game);

            println!("test #{}", i);
            println!("[{},\n{}]", g[0], g[1]);

            assert!(is_isomorphic(&g[0], &g[1], false));
        }
    }
}
