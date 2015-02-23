#![feature(std_misc)]

extern crate "rust-chess" as chess;

//
extern crate piston;
extern crate shader_version;
extern crate glutin_window;

use std::cell::RefCell;
use piston::quack::Set;
use piston::window::{WindowSettings};
use self::piston::input::keyboard::Key;


use shader_version::OpenGL;
use glutin_window::GlutinWindow as Window;
//

use chess::{Game,Network,Inputs,Comm, Events,Event,PlayResult};
use std::thread;

fn main() {
    let mut game = Game::new();

    thread::spawn(move || {
        let svr = Network::new_server();
    });


//
    let window = Window::new(
        OpenGL::_3_2,
        WindowSettings {
            title: "piston-examples/user_input".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    let window = RefCell::new(window);
//    


    let es = Events::new();
    let inp = Inputs::new(window, es.branch());
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
                    Comm::Quit => break 'gameloop, //network thread shutdown
                }
            },
            Event::Inp(inp) => {
                match inp {
                    Inputs::Mouse(btn,pos) => {
                        // todo: check if game is started and mouse-click corresponds to a move selection versus a menu selection!
                        
                        let mv = ((1,1),(2,1));
                        let r: PlayResult = game.play(mv.0,mv.1);
                        println!("{:?}",r);
                        match r {
                            PlayResult::Ok(_) | PlayResult::Check(_,_) => net.send_server(Comm::Move(mv.0,mv.1)),
                            _ => (),
                        }
                    },
                    Inputs::Keyboard(key) => {
                        match key {
                            Key::Q => break 'gameloop,
                            Key::M => println!("!"),
                            _ => (),
                        }
                    },

                    Inputs::Quit => break 'gameloop, //input thread shutdown
                }
            },
            
        }
    }

   /* game.play((1,1),(2,1));
    game.play((6,1),(5,1));
    println!("valid move? {:?}",game.play((0,2),(2,0)));
    println!("{:?}",game);*/

}
