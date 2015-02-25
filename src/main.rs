#![feature(std_misc)]
#![feature(old_io)]

extern crate "rust-chess" as chess;
extern crate glutin;

use chess::{Game,Network,Inputs,Comm,Render, Events,Event};
use std::thread;
use glutin::VirtualKeyCode as VKey;

use std::old_io::timer::sleep;
use std::time::Duration;

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


    let mut rc: Vec<Render> = vec!(); //let's us batch render commands together

    'gameloop: for e in es {
        println!("{:?}",e);
        match e {
            Event::Net(comm) => {
                match comm {
                    Comm::StartGame(g) => {
                        if !game.is_started(){
                            game.start(g.unwrap());
                        }
                        //else { break 'gameloop; }
                    },
                    Comm::Move(f,to) => {
                        let r = game.play(f,to);
                        println!("{:?}",r);
                        if r.is_ok() {
                            rc.push(Render::Animate(r.unwrap()));
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
                        
                        // todo: map pixels to board locations

                        //example move below
                        let mv = ((1,1),(2,1));
                        let r = game.play(mv.0,mv.1);
                        println!("{:?}",r);
                        if r.is_ok() {
                            rc.push(Render::Animate(r.unwrap()));
                            net.send_server(Comm::Move(mv.0,mv.1));
                        }
                        else {
                            // todo: call renderer?
                        }
                    },
                    Inputs::Key(key) => {
                        match key {
                            VKey::Q => {
                                gfx.send(vec!(Render::Quit));
                                // break 'gameloop;
                            },
                            VKey::Pause => { //todo: add state
                                gfx.send(vec!(Render::Pause(true)));
                            },
                            VKey::M => println!("!"),
                            _ => (),
                        }
                    },
                    Inputs::Drag(pos) => (),
                    Inputs::Quit => break 'gameloop, //input thread shutdown
                }
            },
            _ => (),
        }

        gfx.send(rc);
        rc = vec!();
    }

    println!("shutting down main thread");
    net.send_server(Comm::Quit);
    gfx.send(vec!(Render::Quit));
    //let threads die, otherwise glium can get hung up
    sleep(Duration::milliseconds(2000));
}
