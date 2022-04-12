pub mod game;
pub mod abstract_game;
pub mod cup_game;
pub mod project;
pub mod kbsc;
pub mod mkbsc;
pub mod obs_subset;

pub use game::*;
pub use abstract_game::*;
pub use cup_game::*;
pub use project::*;
pub use kbsc::*;
pub use mkbsc::*;
pub use obs_subset::*;

pub use std::{
    fmt,
    rc::Rc,
    hash::{Hash, Hasher},
    collections::{*, hash_map::Entry::*},
    ops::*,
    cell::RefCell
};
pub use fixedbitset::FixedBitSet;
pub use array_init::array_init;
pub use smart_default::SmartDefault;
pub use itertools::Itertools;
pub use arrayvec::ArrayVec;
pub use derive_new::new;
pub use crate::util::cartesian_product;

type Act = usize;
type Agt = usize;
