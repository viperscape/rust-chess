#![feature(std_misc)]

extern crate "rust-chess" as chess;
extern crate glutin;

use chess::{Game,Network,Inputs,Comm,Render, Events,Event,PlayResult};
use std::thread;

fn main() {
    let mut game = Game::new();

    //spawn a loopback test server
    thread::spawn(move || {
        let svr = Network::new_server();
    });


    let es = Events::new();
    let (gfx,inp) = Render::new();
    Inputs::new(inp, es.branch());
    let mut net = Network::new_client(None,es.branch());
    

    'gameloop: for e in es {
        println!("{:?}",e);
        match e {
            Event::Net(comm) => {
                match comm {
                    Comm::StartGame(g) => {
                        if !game.is_started(){
                            game.start(g.unwrap());
                        }
                        else { panic!("game already started!"); }
                    },
                    Comm::Move(f,to) => {
                        let r: PlayResult = game.play(f,to);
                        println!("{:?}",r);
                        match r {
                            PlayResult::Ok(_) | PlayResult::Check(_,_) => (), // todo: call renderer?
                            _ => {//bad game, cheating?
                                net.send_server(Comm::EndGame);
                                break 'gameloop;
                            },
                        }
                    },
                    Comm::EndGame => break 'gameloop,
                    Comm::Quit => break 'gameloop, //network thread shutdown, for now just quit
                }
            },
            Event::Inp(inp) => {
                match inp {
                    Inputs::Click(btn,pos) => {
                        // todo: check if game is started and mouse-click corresponds to a move selection versus a menu selection!
                        
                        let mv = ((1,1),(2,1));
                        let r: PlayResult = game.play(mv.0,mv.1);
                        println!("{:?}",r);
                        match r {
                            PlayResult::Ok(_) | PlayResult::Check(_,_) => net.send_server(Comm::Move(mv.0,mv.1)),
                            _ => (),
                        }
                    },
                    Inputs::Key(key) => {
                        match key {
                            glutin::VirtualKeyCode::Q => {
                                gfx.send(Render::Quit);
                               // break 'gameloop;
                            },
                            glutin::VirtualKeyCode::Pause => { //todo: add state
                                gfx.send(Render::Pause(true));
                            },
                            glutin::VirtualKeyCode::M => println!("!"),
                            _ => (),
                        }
                    },
                    Inputs::Drag(pos) => (),
                    Inputs::Quit => break 'gameloop, //input thread shutdown
                }
            },
            _ => (),
        }
    }

    println!("shutting down main thread");
    net.send_server(Comm::Quit);

    
    //gfx.send(Event::Quit);
    //inp.send(Event::Quit);
    //net.send(Event::Quit);

   /* game.play((1,1),(2,1));
    game.play((6,1),(5,1));
    println!("valid move? {:?}",game.play((0,2),(2,0)));
    println!("{:?}",game);*/

}
