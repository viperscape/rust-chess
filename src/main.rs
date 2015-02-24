#![feature(std_misc)]

extern crate "rust-chess" as chess;
extern crate glutin;

use chess::{Game,Network,Inputs,Comm,Render, Events,Event,MoveType,MoveIllegal,MoveValid};
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
                        let r = game.play(f,to);
                        println!("{:?}",r);
                        if r.is_ok() {
                             // todo: call renderer?
                        }
                        else {
                            //bad game, cheating?
                                net.send_server(Comm::EndGame);
                                break 'gameloop;
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
                        
                        //todo: map pixels to board locations

                        //example move below
                        let mv = ((1,1),(2,1));
                        let r = game.play(mv.0,mv.1);
                        println!("{:?}",r);
                        if r.is_ok() {
                             // todo: call renderer?
                            net.send_server(Comm::Move(mv.0,mv.1));
                        }
                        else {
                            // todo: call renderer?
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
}
