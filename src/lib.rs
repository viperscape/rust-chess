
pub use game::{Game,MoveValid,MoveIllegal};
pub use logic::{Player,Item,MoveType};

pub mod game;
pub mod logic;

pub type Position = (i8,i8); //row,column
pub type Move = (Position,Position);
pub type Capture = (Player,Position);
pub type BoardLayout = [[Option<Player>;8];8];

// algebraic notation for columns
// todo: pieces
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum AN {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}
pub type ANPosition = (AN,i8);
