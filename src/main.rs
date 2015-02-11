extern crate "rust-chess" as chess;
use chess::{Game,Network,Inputs,Comm, Events};
use std::thread::Thread;

fn main() {
    let mut game = Game::new();
    Thread::spawn(move || {
        let svr = Network::new_server();
    });
    
    let es = Events::new();
    let inp = Inputs::new(es.branch());
    let net = Network::new_client(None,es.branch());
    

    
    for e in es {
        println!("{:?}",e);
        break;
    }

    game.play((1,1),(2,1));
    game.play((6,1),(5,1));
    println!("valid move? {:?}",game.play((0,2),(2,0)));
    println!("{:?}",game);

    
}
