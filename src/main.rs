extern crate "rust-chess" as chess;
use chess::{Game};

fn main() {
    let mut game = Game::new();
    println!("valid move? {:?}",game.play((0,0),(7,0)));
    println!("{:?}",game);
}
