#![allow(dead_code)]
#![allow(unused_imports)]
use crate::from_game::dgame;
use crate::from_game;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use petgraph::dot::*;
use std::fmt;
use std::io::Write;

use algo::mkbsc::MKBSC;
use game::{Game, dgame::DGame};
use games::grid_pursuit::GridPursuitGame;
use itertools::Itertools;
use petgraph::visit::{Walker, Dfs, IntoNeighbors, IntoEdges, EdgeRef, Visitable, VisitMap};

use crate::{games::{three_coin_game, cup_game::cup_game}, algo::{project::*, kbsc::KBSC}, game::{strategy::VecStrat, dgame::{*, index::*}}};

#[macro_use]
mod game;
mod games;
mod algo;
#[macro_use]
mod util;
#[macro_use]
mod macros;
mod test;

fn main() {
    //make_cup_game_graphs();
    strategy_synthesis_test();
    //grid_pursuit_test();
}

fn strategy_synthesis_test() {
    let g = cup_game().mkbsc().mkbsc();
    let g1 = dgame(&g.kbsc[0]);
    //let s1 = find_memoryless_strategy(&g1);
    //println!("{:?}\n\n{:?}", g1, s1);
}

fn grid_pursuit_test() {
    let g = GridPursuitGame::<3, 3, 2>::default();
    save_graph(&g, "grid_33");

    let dg = DGame::<2>::from_game(&g, false).unwrap();

    let k = KBSC::new(Project(dg, agent_index(0)));
    save_graph(&k, "grid_33_kbsc");


}

fn print_game<'a, G, const N: usize>(g: G)
where
    G: Game<'a, N>
{
    let dg = DGame::<N>::from_game(g, false).unwrap();
    println!("{:?}", dg);
}

fn save_graph<'a, G, const N: usize>(g: G, path: &str)
where
    G: Game<'a, N>
{
    save_graph_labels(g, path, None);
}

fn save_graph_labels<'a, G, const N: usize>(g: G, path: &str, f: Option<Box<dyn FnMut(&G, &G::Loc) -> Option<String>>>)
where
    G: Game<'a, N>
{
    let path = format!("out/{}.dot", path);
    let path = Path::new(&path);

    let dg = if let Some(f) = f {
        DGame::<N>::from_game_labels(g, false, f).unwrap()
    } else {
        DGame::<N>::from_game(g, false).unwrap()
    };

    let mut edges = HashSet::new();
    let graph = dg.graph.filter_map(|_, n| Some(n), |ei, e| {
        let ep = dg.graph.edge_endpoints(ei).unwrap();
        if edges.contains(&ep) {
            None
        } else {
            edges.insert(ep);
            Some(e)
        }
    });

    let mut file = File::create(path).unwrap();
    write!(file, "{}", Dot::with_config(&graph, &[Config::EdgeNoLabel])).unwrap();
}

fn make_cup_game_graphs() {
    let g = cup_game();
    save_graph(&g, "base");

    let proj0 = Project(g.clone(), agent_index(0));
    save_graph(&proj0, "project_0");
    let proj1 = Project(g.clone(), agent_index(1));
    save_graph(&proj1, "project_1");

    let kbsc0 = KBSC::new(proj0);
    save_graph(&kbsc0, "kbsc_0");
    let kbsc1 = KBSC::new(proj1);
    save_graph(&kbsc1, "kbsc_1");

    let mkbsc = MKBSC::new(g);
    save_graph(&mkbsc, "mkbsc-1");
    let mkbsc2 = MKBSC::new(mkbsc);
    save_graph(&mkbsc2, "mkbsc-2");
    let mkbsc3 = MKBSC::new(mkbsc2);
    save_graph(&mkbsc3, "mkbsc-3");
}
