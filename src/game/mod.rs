pub mod game;
pub mod abstract_game;
pub mod cup_game;
pub mod project;
pub mod kbsc;
pub mod mkbsc;
pub mod obs_subset;
pub mod loc_set;
pub mod constructed_game;
pub mod types;

pub use game::*;
pub use abstract_game::*;
pub use cup_game::CupGame;
pub use project::*;
pub use kbsc::*;
pub use mkbsc::*;
pub use obs_subset::*;
pub use loc_set::*;
pub use constructed_game::*;
pub use types::*;

pub type Act = usize;
pub type Agt = usize;
