use petgraph::graph::node_index;

use crate::game::dgame::{DGame, builder::Builder};

pub mod grid_pursuit;
pub mod cup_game;

#[allow(dead_code)]

pub fn three_coin_game() -> DGame<u8, 1> {
    let mut g = Builder::default();

    g.l0(8);

    for i in 0..9 {
        g.add_node1(i == 7, match i {
            0 => 0,
            1 | 2 | 4 => 1,
            6 | 5 | 3 => 2,
            7 => 3,
            8 => 4,
            _ => unreachable!()
        });
    }

    for i in 0..9 {
        match i {
            0 | 7 => {
                g.add_edge1(i, i, [0, 1, 2]);
            },
            8 => {
                g.add_edge1(i, 6, [0, 1, 2]);
                g.add_edge1(i, 5, [0, 1, 2]);
                g.add_edge1(i, 3, [0, 1, 2]);
            },
            _ => {
                g.add_edge1(i, i^1, [0]);
                g.add_edge1(i, i^2, [1]);
                g.add_edge1(i, i^4, [2]);
            }
        }
    }

    g.build()
}
