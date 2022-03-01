use petgraph::graph::node_index;

use crate::game::dgame::DGame;

pub fn three_coin_game() -> DGame<u8> {
    let mut g = DGame::default();

    g.l0 = node_index(8);

    for i in 0..9 {
        g.add_node(i == 7, match i {
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
                g.add_edge(i, i, [0, 1, 2]);
            },
            8 => {
                g.add_edge(i, 6, [0]);
                g.add_edge(i, 5, [1]);
                g.add_edge(i, 3, [2]);
            },
            _ => {
                g.add_edge(i, i^1, [0]);
                g.add_edge(i, i^2, [1]);
                g.add_edge(i, i^4, [2]);
            }
        }
    }

    g
}
