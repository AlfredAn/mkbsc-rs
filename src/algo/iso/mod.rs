use vec_map::VecMap;

use crate::{Loc, loc};

use super::graph::Graph;

use array_init::array_init;

#[cfg(test)]
mod test;

// based on the VF2 algorithm
// see the paper "A (Sub)Graph Isomorphism Algorithm for Matching Large Graphs"
// by Luigi P. Cordella, Pasquale Foggia, Carlo Sansone, and Mario Vento

#[derive(Clone, Copy)]
struct Depth(usize);

struct State<'g, G: Graph> {
    // graphs being analyzed
    g: [&'g G; 2],

    // partial mapping between the two graphs
    m: [VecMap<Loc>; 2],

    // out/in sets for each graph
    out_set: [VecMap<Depth>; 2],
    in_set: [VecMap<Depth>; 2],

    // T_out[i] is all the elements that are in out_set[i] but not in m[i].
    // T_in[i] is all the elements that are in in_set[i] but not in m[i].
}

impl<'g, G: Graph> Clone for State<'g, G> {
    fn clone(&self) -> Self {
        Self { g: self.g.clone(), m: self.m.clone(), out_set: self.out_set.clone(), in_set: self.in_set.clone() }
    }
}

impl<'g, G: Graph> State<'g, G> {
    // check if the mapping covers all of g[1]
    fn is_fully_matched(&self) -> bool {
        assert_eq!(self.m[0].len(), self.m[1].len());
        self.m[0].len() == self.g[1].size()
    }

    fn candidate_pairs(&self) -> Vec<(Loc, Loc)> {
        let mut result = vec![];

        // adds all pairs satisfying the given condition to the result vector
        macro_rules! find_pairs {
            ($condition:expr) => {
                for l0 in (0..self.g[0].size()).map(|x| loc(x)) {
                    if !($condition)(0, l0) { continue; }
        
                    for l1 in (0..self.g[1].size()).map(|x| loc(x)) {
                        if !($condition)(1, l1) { continue; }
                        
                        result.push((l0, l1));
                    }
                }
            };
        }

        if self.tout_len(0) != self.tout_len(1)
        || self.tin_len(0) != self.tin_len(1) {
            return result;
        }

        if self.tout_len(0) > 0 && self.tout_len(1) > 0 {
            //println!("pairs a");
            find_pairs!(|i, l| self.is_in_tout(i, l));
        } else if self.tin_len(0) > 0 && self.tin_len(1) > 0 {
            //println!("pairs b");
            find_pairs!(|i, l| self.is_in_tin(i, l));
        } else {
            //println!("pairs c");
            find_pairs!(|i, l| !self.is_mapped(i, l));
        }

        result
    }

    fn is_mapping_feasible(&self, l0: Loc, l1: Loc) -> bool {
        //println!("is feasible?");

        let l = [l0, l1];

        // R_self
        // check that the number of self edges is equal
        //println!("R_self");
        if self.g[0].edges_between(l0, l0).count() != self.g[1].edges_between(l1, l1).count() {
            return false;
        }

        // R_succ
        // for each successor of l0 in g0 that is already mapped,
        // there exists a successor of l1 in g1 that is mapped to it.
        // (and vice versa)
        //println!("R_succ");
        for i in [0, 1] {
            'outer: for (_, succ_a) in self.g[i].successors(l[i]) {
                if !self.is_mapped(i, succ_a) { continue; }
                for (_, succ_b) in self.g[1-i].successors(l[1-i]) {
                    if self.get_mapping(i, succ_a) == Some(succ_b) {
                        // match found, checking next...
                        continue 'outer;
                    }
                }
                // no match found
                return false;
            }
        }

        // R_pred
        // same as R_succ but for predecessors
        //println!("R_pred");
        for i in [0, 1] {
            'outer: for (_, pred_a) in self.g[i].predecessors(l[i]) {
                if !self.is_mapped(i, pred_a) { continue; }
                for (_, pred_b) in self.g[1-i].predecessors(l[1-i]) {
                    if self.get_mapping(i, pred_a) == Some(pred_b) {
                        // match found, checking next...
                        continue 'outer;
                    }
                }
                // no match found
                return false;
            }
        }

        // R_out
        // count number of successors in T_out for each graph
        //println!("R_out");
        let out_count: [usize; 2] = array_init(|i| {
            self.g[i]
                .successors(l[i])
                .filter(|&(_, succ)| self.is_in_tout(i, succ))
                .count()
        });
        if out_count[0] != out_count[1] {
            return false;
        }

        // R_in
        // count number of predecessors in T_in for each graph
        //println!("R_in");
        let in_count: [usize; 2] = array_init(|i| {
            self.g[i]
                .predecessors(l[i])
                .filter(|&(_, pred)| self.is_in_tin(i, pred))
                .count()
        });
        if in_count[0] != in_count[1] {
            return false;
        }
        
        // all tests passed
        true
    }

    fn add_mapping(&mut self, l0: Loc, l1: Loc) {
        //println!("add mapping ({}, {})", l0, l1);

        assert!(self.get_mapping(0, l0).is_none());
        assert!(self.get_mapping(1, l1).is_none());

        assert!(self.m[0].len() == self.m[1].len());

        let depth = Depth(self.m[0].len());
        let l = [l0, l1];

        for i in [0, 1] {
            self.m[i].insert(l[i].index(), l[1-i]);

            self.out_set[i].insert(l[i].index(), depth);
            self.in_set[i].insert(l[i].index(), depth);

            for (_, l2) in self.g[i].successors(l[i]) {
                self.out_set[i].insert(l2.index(), depth);
            }
            for (_, l2) in self.g[i].predecessors(l[i]) {
                self.in_set[i].insert(l2.index(), depth);
            }
        }

        assert!(self.get_mapping(0, l0) == Some(l1));
        assert!(self.get_mapping(1, l1) == Some(l0));
    }

    fn has_mapping(&self, l0: Loc, l1: Loc) -> bool {
        self.get_mapping(0, l0) == Some(l1)
    }

    fn get_mapping(&self, i: usize, l: Loc) -> Option<Loc> {
        self.m[i].get(l.index()).copied()
    }

    fn tout_len(&self, i: usize) -> usize { self.out_set[i].len() - self.m[i].len() }
    fn tin_len (&self, i: usize) -> usize { self.in_set [i].len() - self.m[i].len() }

    fn is_mapped(&self, i: usize, l: Loc) -> bool {
        self.m[i].contains_key(l.index())
    }

    fn is_in_tout(&self, i: usize, l: Loc) -> bool {
        self.out_set[i].contains_key(l.index()) && !self.is_mapped(i, l)
    }
    fn is_in_tin(&self, i: usize, l: Loc) -> bool {
        self.in_set[i].contains_key(l.index()) && !self.is_mapped(i, l)
    }
}

impl<'g, G: Graph> State<'g, G> {
    fn new(g: [&'g G; 2]) -> Self {
        Self {
            g,
            m:       [VecMap::with_capacity(g[0].size()), VecMap::with_capacity(g[1].size())],
            out_set: [VecMap::with_capacity(g[0].size()), VecMap::with_capacity(g[1].size())],
            in_set:  [VecMap::with_capacity(g[0].size()), VecMap::with_capacity(g[1].size())],
        }
    }
}

pub fn is_isomorphic<G: Graph>(g0: &G, g1: &G) -> bool {
    if g0.size() != g1.size() {
        return false;
    }

    let g = [g0, g1];
    let s = State::new(g);
    iso_match(s)
}

fn iso_match<G: Graph>(s: State<G>) -> bool {
    //println!("iso_match (depth {})", s.m[0].len());

    if s.is_fully_matched() {
        //println!("fully matched");
        return true;
    }

    for (l0, l1) in s.candidate_pairs() {
        //println!("checking ({}, {})", l0, l1);

        if s.is_mapping_feasible(l0, l1) {
            let mut s2 = s.clone();
            s2.add_mapping(l0, l1);

            if iso_match(s2) {
                return true;
            }

            //println!("backtracking...")
        }
    };

    

    false
}
