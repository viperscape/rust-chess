extern crate "rust-chess" as chess;
use chess::{Game,Network,Inputs,Comm, Events};
use std::thread::Thread;

fn main() {
    let mut game = Game::new();
    Thread::spawn(move || {
        let svr = Network::new_server();
    });
    
    let inp = Inputs::new();
    let net = Network::new_client(None);
    

    let events = Events::new(inp,net);

    game.play((1,1),(2,1));
    game.play((6,1),(5,1));
    println!("valid move? {:?}",game.play((0,2),(2,0)));
    println!("{:?}",game);

    
}
