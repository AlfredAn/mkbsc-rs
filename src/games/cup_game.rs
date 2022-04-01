use crate::game::dgame::{DGame, builder::Builder, generic_builder::GenericBuilder};

#[allow(dead_code)]

pub fn cup_game() -> DGame<u8, 2> {
    _cup_game().unwrap()
}

fn _cup_game() -> anyhow::Result<DGame<u8, 2>> {
    let mut gm = GenericBuilder::default();

    gm.node("start", false)?;
    gm.node("bad", false)?;
    gm.node("good", false)?;
    gm.node("lose", false)?;
    gm.node("win", true)?;

    gm.l0("start")?;

    let (g, s, l) = (0, 1, 2);

    gm.edge("start", "bad", [[g, g]])?;
    gm.edge("start", "good", [[g, g]])?;
    gm.edge("bad", "good", [[s, s]])?;
    gm.edge("good", "good", [[s, s]])?;
    gm.edge("bad", "lose", [[l, l], [l, s], [s, l]])?;
    gm.edge("good", "lose", [[s, l], [l, s]])?;
    gm.edge("good", "win", [[l, l]])?;
    gm.edge("win", "win", [[l, l]])?;
    gm.edge("lose", "lose", [[s, s]])?;

    gm.obs(0, 0, ["start"]);
    gm.obs(1, 0, ["bad"]);
    gm.obs(2, 0, ["good"]);
    gm.obs(3, 0, ["lose"]);
    gm.obs(4, 0, ["win"]);

    gm.obs(0, 1, ["start"]);
    gm.obs(1, 1, ["bad", "good"]);
    gm.obs(2, 1, ["lose"]);
    gm.obs(3, 1, ["win"]);

    gm.labels(Box::new(|&l| match l {
        "start" => "S",
        "good" => "G",
        "bad" => "B",
        "lose" => "L",
        "win" => "W",
        _ => unreachable!()
    }.into()));
    gm.build()
}
