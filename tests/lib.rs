extern crate chess;
use chess::{Game};

#[test]
fn test_basic() {
    let mut game = Game::new();

    game.play((1,1),(2,1));
    game.play((6,1),(5,1));
    assert!(game.play((0,2),(2,0)).is_ok());
}
