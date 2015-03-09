#![feature(std_misc)]
#![feature(core)]
#![feature(old_io)]


extern crate "rustc-serialize" as rustc_serialize;

extern crate glutin;
#[macro_use]
extern crate glium;


pub use game::{Game,MoveValid,MoveIllegal};
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
pub mod glium_support;

pub type Position = (i8,i8); //change to u8 when rust gets changed!
pub type Move = (Position,Position);
pub type Capture = (Player,Position);
pub type BoardLayout = [[Option<Player>;8];8];
