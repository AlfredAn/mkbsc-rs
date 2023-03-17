use std::ops::Index;

use anyhow::{anyhow, ensure, bail};
use array_init::array_init;
use arrayvec::ArrayVec;
use disjoint_sets::UnionFind;
use fixedbitset::FixedBitSet;
use itertools::Itertools;
use superslice::Ext;

use super::*;

#[derive(Default, Clone, Debug)]
struct AdjGraph {
    nodes: Vec<Node>,
}

impl AdjGraph {
    fn add_node(&mut self) -> usize {
        self.nodes.push(Node::default());
        self.size() - 1
    }
    
    fn add_edge(&mut self, from: usize, to: usize) {
        self.nodes[from].succ.push(to);
        self.nodes[to].pred.push(from);
    }

    fn clear_edges(&mut self) {
        for node in &mut self.nodes {
            node.succ.clear();
            node.pred.clear();
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Node {
    succ: Vec<usize>,
    pred: Vec<usize>,
}

impl Graph for AdjGraph {
    type EdgeData = ();

    fn size(&self) -> usize {
        self.nodes.len()
    }

    fn successors(&self, from: Loc) -> Box<dyn Iterator<Item=(Self::EdgeData, Loc)> + '_> {
        Box::new(
            self.nodes[from.index()].succ.iter()
                .map(|&i| ((), loc(i))) 
        )
    }

    fn predecessors(&self, to: Loc) -> Box<dyn Iterator<Item=(Self::EdgeData, Loc)> + '_> {
        Box::new(
            self.nodes[to.index()].pred.iter()
                .map(|&i| ((), loc(i))) 
        )
    }
}

#[derive(Default)]
struct BitGraph<const N: usize>(u32);

impl<const N: usize> BitGraph<N> {
    fn idx(from: usize, to: usize) -> usize {
        from * N + to
    }

    fn get(&self, from: usize, to: usize) -> bool {
        self.0 & (1 << Self::idx(from, to)) != 0
    }

    fn set(&mut self, from: usize, to: usize, val: bool) {
        let bit = 1 << Self::idx(from, to);
        self.0 &= !bit; // zero the bit
        self.0 |= bit * (val as u32); // set new value
    }

    fn permute(&self, p: &[usize]) -> Self {
        let mut res = Self::default();

        for i in 0..N {
            for j in 0..N {
                let (i2, j2) = (p[i], p[j]);

                res.set(i2, j2, self.get(i, j));
            }
        }

        res
    }
}

impl<const N: usize> Graph for BitGraph<N> {
    type EdgeData = ();

    fn successors(&self, from: Loc) -> Box<dyn Iterator<Item=(Self::EdgeData, Loc)> + '_> {
        Box::new(
            (0..N)
                .filter_map(move |to|
                    if self.get(from.index(), to) {
                        Some(((), loc(to)))
                    } else {
                        None
                    }
            )
        )
    }

    fn predecessors(&self, to: Loc) -> Box<dyn Iterator<Item=(Self::EdgeData, Loc)> + '_> {
        Box::new(
            (0..N)
                .filter_map(move |from|
                    if self.get(from, to.index()) {
                        Some(((), loc(to)))
                    } else {
                        None
                    }
            )
        )
    }

    fn size(&self) -> usize {
        N
    }
}

fn parse_graph(input: &str) -> anyhow::Result<AdjGraph> {
    let mut g = AdjGraph::default();
    let mut parser = input
        .split_whitespace()
        .map(|s| s.parse::<usize>());

    let size = parser.next()
        .ok_or_else(|| anyhow!("missing size"))??;

    for _ in 0..size {
        g.add_node();
    }

    for i in 0..size {
        for j in 0..size {
            let x = parser.next()
                .ok_or_else(|| anyhow!("unexpected end of input"))??;

            match x {
                0 => (),
                1 => g.add_edge(i, j),
                _ => bail!("invalid input"),
            }
        }
    }

    ensure!(parser.next().is_none(), "input too long");

    Ok(g)
}

#[test]
fn test_trivial() {
    let (mut g0, mut g1): (AdjGraph, AdjGraph) = Default::default();
    assert!(is_isomorphic(&g0, &g1));

    g0.add_node();
    assert!(!is_isomorphic(&g0, &g1));

    g1.add_node();
    assert!(is_isomorphic(&g0, &g1));

    g0.add_node();
    g1.add_node();

    g0.add_edge(0, 1);
    assert!(!is_isomorphic(&g0, &g1));

    g1.add_edge(1, 0);
    assert!(is_isomorphic(&g0, &g1));
}

#[test]
fn test_add_mapping() {
    let mut g = AdjGraph::default();
    g.add_node();
    g.add_node();
    g.add_edge(0, 1);

    let mut s = State::new([&g, &g]);
    s.add_mapping(loc(0), loc(1));

    assert_eq!(s.get_mapping(0, loc(0)), Some(loc(1)));
    assert_eq!(s.get_mapping(0, loc(1)), None);
    assert_eq!(s.get_mapping(1, loc(0)), None);
    assert_eq!(s.get_mapping(1, loc(1)), Some(loc(0)));

    assert!(!s.is_in_tout(0, loc(0)));
    assert!( s.is_in_tout(0, loc(1)));
    assert!(!s.is_in_tout(1, loc(0)));
    assert!(!s.is_in_tout(1, loc(1)));

    assert!(!s.is_in_tin(0, loc(0)));
    assert!(!s.is_in_tin(0, loc(1)));
    assert!( s.is_in_tin(1, loc(0)));
    assert!(!s.is_in_tin(1, loc(1)));
}

#[test]
fn test_self_edge() -> anyhow::Result<()> {
    let g0 = parse_graph("1  1")?;
    let g1 = parse_graph("1  0")?;

    assert!(!is_isomorphic(&g0, &g1));

    Ok(())
}

#[test]
fn test_exhaustive() {
    do_test_exhaustive::<0>();
    do_test_exhaustive::<1>();
    do_test_exhaustive::<2>();
    do_test_exhaustive::<3>();
}

fn do_test_exhaustive<const N: usize>() {
    println!("testing N={}", N);

    let bits = N * N;
    let n = 1 << bits;
    let mut uf = UnionFind::<u32>::new(n);

    for x in 0..n {
        let mut perm: [usize; N] = array_init(|i| i);
        let graph: BitGraph<N> = BitGraph(x as u32);

        loop {
            let permuted = graph.permute(&perm);
            uf.union(graph.0, permuted.0);

            if !perm.next_permutation() { break; }
        }
    }

    for x in 0..n {
        let g0: BitGraph<N> = BitGraph(x as u32);
        for y in 0..n {
            let g1: BitGraph<N> = BitGraph(y as u32);

            println!("{:09b} = {:09b} ?", g0.0, g1.0);

            assert_eq!(uf.equiv(g0.0, g1.0), is_isomorphic(&g0, &g1));
        }
    }

    println!("{}", uf.to_vec().iter().unique().count());
}

// tests from https://www.dharwadker.org/tevet/isomorphism/

#[test]
fn test_1() -> anyhow::Result<()> {
    let mut g0 = parse_graph(
        "8
        0 0 0 1 1 1 1 1 
        0 0 0 1 1 1 1 1 
        0 0 0 1 1 1 1 1 
        1 1 1 0 0 1 1 1 
        1 1 1 0 0 1 1 1 
        1 1 1 1 1 0 0 0 
        1 1 1 1 1 0 0 0 
        1 1 1 1 1 0 0 0")?;
    let g1 = parse_graph(
        "8
        0 1 1 1 1 1 1 0 
        1 0 1 0 0 1 1 1 
        1 1 0 1 1 0 0 1 
        1 0 1 0 0 1 1 1 
        1 0 1 0 0 1 1 1 
        1 1 0 1 1 0 0 1 
        1 1 0 1 1 0 0 1 
        0 1 1 1 1 1 1 0")?;

    assert!(is_isomorphic(&g0, &g1));

    g0.add_edge(0, 0);
    //assert!(!is_isomorphic(&g0, &g1));

    Ok(())
}

#[test]
fn test_petersen() -> anyhow::Result<()> {
    let g0 = parse_graph(
        "10 
        0 1 0 0 1 0 1 0 0 0 
        1 0 1 0 0 0 0 1 0 0 
        0 1 0 1 0 0 0 0 1 0 
        0 0 1 0 1 0 0 0 0 1 
        1 0 0 1 0 1 0 0 0 0 
        0 0 0 0 1 0 0 1 1 0 
        1 0 0 0 0 0 0 0 1 1 
        0 1 0 0 0 1 0 0 0 1 
        0 0 1 0 0 1 1 0 0 0 
        0 0 0 1 0 0 1 1 0 0")?;
    let g1 = parse_graph(
        "10
        0 0 0 1 0 1 0 0 0 1 
        0 0 0 1 1 0 1 0 0 0 
        0 0 0 0 0 0 1 1 0 1 
        1 1 0 0 0 0 0 1 0 0
        0 1 0 0 0 0 0 0 1 1 
        1 0 0 0 0 0 1 0 1 0 
        0 1 1 0 0 1 0 0 0 0 
        0 0 1 1 0 0 0 0 1 0 
        0 0 0 0 1 1 0 1 0 0 
        1 0 1 0 1 0 0 0 0 0")?;

    assert!(is_isomorphic(&g0, &g1));

    Ok(())
}

#[test]
fn test_icosahedron() -> anyhow::Result<()> {
    let g0 = parse_graph(
        "12
        0 1 1 0 0 1 1 1 0 0 0 0 
        1 0 1 1 1 1 0 0 0 0 0 0 
        1 1 0 1 0 0 0 1 1 0 0 0 
        0 1 1 0 1 0 0 0 1 1 0 0 
        0 1 0 1 0 1 0 0 0 1 1 0 
        1 1 0 0 1 0 1 0 0 0 1 0 
        1 0 0 0 0 1 0 1 0 0 1 1 
        1 0 1 0 0 0 1 0 1 0 0 1 
        0 0 1 1 0 0 0 1 0 1 0 1 
        0 0 0 1 1 0 0 0 1 0 1 1 
        0 0 0 0 1 1 1 0 0 1 0 1 
        0 0 0 0 0 0 1 1 1 1 1 0")?;
    let g1 = parse_graph(
        "12
        0 0 1 0 0 1 0 0 1 1 0 1 
        0 0 0 1 1 0 0 1 1 0 0 1 
        1 0 0 0 0 1 0 1 1 0 1 0 
        0 1 0 0 1 0 1 1 0 0 1 0 
        0 1 0 1 0 0 1 0 0 1 0 1 
        1 0 1 0 0 0 1 0 0 1 1 0 
        0 0 0 1 1 1 0 0 0 1 1 0 
        0 1 1 1 0 0 0 0 1 0 1 0 
        1 1 1 0 0 0 0 1 0 0 0 1 
        1 0 0 0 1 1 1 0 0 0 0 1 
        0 0 1 1 0 1 1 1 0 0 0 0 
        1 1 0 0 1 0 0 0 1 1 0 0 ")?;

    assert!(is_isomorphic(&g0, &g1));

    Ok(())
}

#[test]
fn test_ramsey() -> anyhow::Result<()> {
    let g0 = parse_graph(
        "17
        0 1 1 0 1 0 0 0 1 1 0 0 0 1 0 1 1 
        1 0 1 1 0 1 0 0 0 1 1 0 0 0 1 0 1 
        1 1 0 1 1 0 1 0 0 0 1 1 0 0 0 1 0 
        0 1 1 0 1 1 0 1 0 0 0 1 1 0 0 0 1 
        1 0 1 1 0 1 1 0 1 0 0 0 1 1 0 0 0 
        0 1 0 1 1 0 1 1 0 1 0 0 0 1 1 0 0
        0 0 1 0 1 1 0 1 1 0 1 0 0 0 1 1 0 
        0 0 0 1 0 1 1 0 1 1 0 1 0 0 0 1 1 
        1 0 0 0 1 0 1 1 0 1 1 0 1 0 0 0 1 
        1 1 0 0 0 1 0 1 1 0 1 1 0 1 0 0 0 
        0 1 1 0 0 0 1 0 1 1 0 1 1 0 1 0 0 
        0 0 1 1 0 0 0 1 0 1 1 0 1 1 0 1 0 
        0 0 0 1 1 0 0 0 1 0 1 1 0 1 1 0 1 
        1 0 0 0 1 1 0 0 0 1 0 1 1 0 1 1 0 
        0 1 0 0 0 1 1 0 0 0 1 0 1 1 0 1 1 
        1 0 1 0 0 0 1 1 0 0 0 1 0 1 1 0 1 
        1 1 0 1 0 0 0 1 1 0 0 0 1 0 1 1 0 ")?;
    let g1 = parse_graph(
        "17
        0 1 1 0 1 0 0 0 1 1 0 0 0 1 0 1 1 
        1 0 1 1 0 1 0 0 0 1 1 0 0 0 1 0 1 
        1 1 0 1 1 0 1 0 0 0 1 1 0 0 0 1 0 
        0 1 1 0 1 1 0 1 0 0 0 1 1 0 0 0 1 
        1 0 1 1 0 1 1 0 1 0 0 0 1 1 0 0 0 
        0 1 0 1 1 0 1 1 0 1 0 0 0 1 1 0 0
        0 0 1 0 1 1 0 1 1 0 1 0 0 0 1 1 0 
        0 0 0 1 0 1 1 0 1 1 0 1 0 0 0 1 1 
        1 0 0 0 1 0 1 1 0 1 1 0 1 0 0 0 1 
        1 1 0 0 0 1 0 1 1 0 1 1 0 1 0 0 0 
        0 1 1 0 0 0 1 0 1 1 0 1 1 0 1 0 0 
        0 0 1 1 0 0 0 1 0 1 1 0 1 1 0 1 0 
        0 0 0 1 1 0 0 0 1 0 1 1 0 1 1 0 1 
        1 0 0 0 1 1 0 0 0 1 0 1 1 0 1 1 0 
        0 1 0 0 0 1 1 0 0 0 1 0 1 1 0 1 1 
        1 0 1 0 0 0 1 1 0 0 0 1 0 1 1 0 1 
        1 1 0 1 0 0 0 1 1 0 0 0 1 0 1 1 0 ")?;

    assert!(is_isomorphic(&g0, &g1));

    Ok(())
}

/*#[test]
fn test_dodecahedron() -> anyhow::Result<()> {
    let g0 = parse_graph(
        "20
        0 1 0 0 1 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 
        1 0 1 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 
        0 1 0 1 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 
        0 0 1 0 1 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0
        1 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
        0 0 0 0 1 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 
        0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 1 0 0 0 
        0 0 0 1 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 
        0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 1 0 0 
        0 0 1 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 0 0 
        0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 1 0 
        0 1 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 1 
        1 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 
        0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 1 0 0 0 0 
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 1 
        0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 1 0 1 0 0 
        0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 1 0 
        0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 0 1 
        0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0 1 0 
       ")?;
    let g1 = parse_graph(
        "20
        0 0 0 0 0 1 0 1 0 0 0 0 0 1 0 0 0 0 0 0 
        0 0 0 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
        0 0 0 1 0 0 0 0 0 0 0 0 0 0 1 0 0 0 1 0 
        0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 
        0 1 0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0 0 0 
        1 1 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 1 0 
        1 0 0 0 0 0 0 0 0 1 1 0 0 0 0 0 0 0 0 0 
        0 0 0 0 1 0 0 0 0 0 0 1 0 0 0 0 0 1 0 0 
        0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 1 0 0 0 0 
        0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 0 0 0 0 1 
        0 0 0 0 0 0 0 0 1 0 0 0 1 0 0 0 1 0 0 0 
        0 0 0 0 0 0 0 0 0 1 0 1 0 1 0 0 0 0 0 0 
        1 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 
        0 0 1 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0 0 0 
        0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 0 0 1 
        0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 1 0 0 0 0 
        0 0 0 1 0 0 1 0 1 0 0 0 0 0 0 0 0 0 0 0 
        0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 1 
        0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 1 0 ")?;

    assert!(is_isomorphic(&g0, &g1));

    Ok(())
}*/

