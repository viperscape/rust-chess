extern crate chess;
use chess::{Game,Player,Item,AN};

#[test]
fn test_basic() {
    let mut game = Game::new();

    game.play((1,1),(2,1));
    assert_eq!(game.get_active(),Player::Black(Item::Pawn));
    game.play((6,1),(5,1));
    assert_eq!(game.get_active(),Player::White(Item::Pawn));
}

#[test]
/// rome: 1619, famous chess game
fn test_greco_vs_nn() {
    let mut game = Game::new();

    game.play_an((AN::E,2),(AN::E,4));
    game.play_an((AN::B,7),(AN::B,6));
    
    game.play_an((AN::D,2),(AN::D,4));
    game.play_an((AN::C,8),(AN::B,7));

    game.play_an((AN::F,1),(AN::D,3));
    game.play_an((AN::F,7),(AN::F,5));

    let r = game.play_an((AN::E,4),(AN::F,5)); //cap
    assert!(r.unwrap().cap.is_some());
    let r = game.play_an((AN::B,7),(AN::G,2)); //cap
    assert!(r.unwrap().cap.is_some());

    let r = game.play_an((AN::D,1),(AN::H,5)); //check
    assert!(r.unwrap().check.is_some());
    assert!(game.in_check().is_some());
    game.play_an((AN::G,7),(AN::G,6)); //block check
    assert!(!game.in_check().is_some());

    let r = game.play_an((AN::F,5),(AN::G,6)); //cap
    assert!(r.unwrap().cap.is_some());
    assert!(game.play_an((AN::G,8),(AN::F,6)).is_ok()); //black knight moves

    let r = game.play_an((AN::G,6),(AN::H,7)); //cap
    assert!(r.unwrap().cap.is_some());
    let r = game.play_an((AN::F,6),(AN::H,5)); //black knight cap
    assert!(r.unwrap().cap.is_some());

    let r = game.play_an((AN::D,3),(AN::G,6)); //white bishop check mates
    assert!(game.in_check().is_some());

    // todo: validate checkmate
}
