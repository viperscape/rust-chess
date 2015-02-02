extern crate "rust-chess" as chess;
use chess::{Game};

fn main() {
    let mut game = Game::new();
    println!("valid move? {:?}",game.play((1,0),(3,0)));
    println!("{:?}",game);
}
