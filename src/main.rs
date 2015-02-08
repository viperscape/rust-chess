extern crate "rust-chess" as chess;
use chess::{Game};

fn main() {
    let mut game = Game::new();
    game.play((1,1),(2,1));
    game.play((6,1),(5,1));
    println!("valid move? {:?}",game.play((0,2),(2,0)));
    println!("{:?}",game);
}
