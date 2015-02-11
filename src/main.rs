extern crate "rust-chess" as chess;
use chess::{Game,Network,Inputs,Comm, Events,Event,PlayResult};
use std::thread::Thread;

fn main() {
    let mut game = Game::new();

    Thread::spawn(move || {
        let svr = Network::new_server();
    });
    
    let es = Events::new();
    let inp = Inputs::new(es.branch());
    let mut net = Network::new_client(None,es.branch());
    

    let mut i = 0;
    for e in es {
        println!("{:?}",e);
        match e {
            Event::Net(comm) => {
                match comm {
                   Comm::StartGame(g) => {
                        game.start(g.unwrap());
                   },
                    _ => (),
                }
            },
            Event::Inp(inp) => {
                match inp {
                    Inputs::Mouse1 => {
                        let mv = ((1,1),(2,1));
                        let r: PlayResult = game.play(mv.0,mv.1);
                        net.send(Comm::Move(mv.0,mv.1));
                    },
                    _ => (),
                }
            },
            
        }
        
        i+=1;
        if i == 2 { break; }
    }

   /* game.play((1,1),(2,1));
    game.play((6,1),(5,1));
    println!("valid move? {:?}",game.play((0,2),(2,0)));
    println!("{:?}",game);*/

    
}
