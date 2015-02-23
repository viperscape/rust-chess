#![feature(std_misc)]
extern crate "rustc-serialize" as rustc_serialize;

pub use game::{Game,PlayResult};
pub use logic::{Player,Item,MoveType};
pub use network::{Network,Comm};
pub use input::{Inputs};
pub use events::{Events,Event};
pub use render::{Render};

pub mod game;
pub mod logic;
pub mod network;
pub mod input;
pub mod events;
pub mod render;

pub type Position = (usize,usize); //change to u8 when rust gets changed!
pub type Move = (Position,Position);
