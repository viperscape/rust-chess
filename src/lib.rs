
pub use game::{Game,MoveValid,MoveIllegal};
pub use logic::{Player,Item,MoveType};

pub mod game;
pub mod logic;

pub type Position = (i8,i8); //change to u8 when rust gets changed!
pub type Move = (Position,Position);
pub type Capture = (Player,Position);
pub type BoardLayout = [[Option<Player>;8];8];
