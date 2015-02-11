extern crate wire;

use super::{Game,Position,Event,Player,Item};
use std::thread::Thread;
use self::wire::{SizeLimit,tcp};
use std::rand;

use std::sync::mpsc::{channel,Receiver,Sender};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug,RustcDecodable, RustcEncodable,Copy)]
pub enum Comm {
    Move(Position,Position), //from, to
    StartGame(Option<u64>),
    EndGame(u64),
    //Chat(String),
}

struct NetGame {
    white: Option<tcp::OutTcpStream<Comm>>,
    black: Option<tcp::OutTcpStream<Comm>>,
    moves: Vec<(Position,Position)>,
    id: u64,
}


// all open games
struct Games(HashMap<u64,NetGame>);
impl Games {
    fn new () -> Games {
        Games(HashMap::new())
    }

    fn insert (&mut self, id:u64) {
        self.0.insert(id,NetGame { white:None,
                                   black:None,
                                   moves:vec!(),
                                   id:id });
    }

    fn attach (&mut self, id:u64, p:tcp::OutTcpStream<Comm>) -> Option<Player> {
        match self.0.get_mut(&id) {
            Some(g) => {
                if g.white.is_some() {
                    g.black = Some(p);
                    Some(Player::Black(Item::Pawn))
                }
                else {
                    g.white = Some(p);
                    Some(Player::White(Item::Pawn))
                }
            },
            None => None,
        }
    }

    fn update (&mut self, id:u64, m: (Position,Position)) {
        match self.0.get_mut(&id) {
            Some(g) => {g.moves.push(m);},
            None => (),
        }
    }

    fn get_last (&self, id:u64) -> Option<(Position,Position)> {
        match self.0.get(&id) {
            Some(g) => {
                let s = g.moves.len()-1;
                Some(g.moves[s])
            },
            None => None,
        }
    }

    fn get_moves (&self, id:u64) -> Option<&[(Position,Position)]> {
        match self.0.get(&id) {
            Some(g) => {
                Some(g.moves.as_slice())
            },
            None => None,
        }
    }
}

pub struct Network(tcp::OutTcpStream<Comm>);
impl Network {
    pub fn new_server (){

        let (listener,_) = wire::listen_tcp(("localhost", 9999)).unwrap();

        let games = Arc::new(Mutex::new(Games::new()));
        

        for conn in listener.into_blocking_iter() {
            let _games = games.clone();

            Thread::spawn(move || {
                let (i, mut o) = wire::upgrade_tcp(conn,
                                                   SizeLimit::Bounded(1000),
                                                   SizeLimit::Bounded(1000));

                let mut id: Option<u64> = None;

                for n in i.into_blocking_iter() {
                    match n {
                        Comm::Move(f,t) => {
                            
                        },
                        Comm::StartGame(g) => {
                            if let Some(gid) = g {
                                id = Some(gid);

                                //lookup game id in table
                                let r = {
                                    let mut gl = _games.lock().unwrap();
                                    *gl.attach(gid,o.clone())
                                };

                                o.send(&Comm::StartGame(g));
                                
                            }
                            else { //generate a game id
                                let ng = rand::random::<u64>();
                                o.send(&Comm::StartGame(Some(ng)));
                                id = Some(ng);
                            } 
                        }, 
                        Comm::EndGame(gid) => {
                            o.send(&Comm::EndGame(gid));
                            break;
                        }
                    }
                    
                }
            });
        }
    }


    pub fn new_client (gid: Option<u64>, t: Sender<Event>) -> Network {
        let (i, mut o) = wire::connect_tcp(("localhost",9999),
                                           SizeLimit::Bounded(1000),
                                           SizeLimit::Bounded(1000)).unwrap();

        o.send(&Comm::StartGame(gid));

        Thread::spawn(move || {
            for n in i.into_blocking_iter() {
                match n {
                    Comm::Move(f,to) => { t.send(Event::Net(Comm::Move(f,to))); },
                    Comm::EndGame(gid) => {
                        t.send(Event::Net(Comm::EndGame(gid)));
                        break;
                    },
                    Comm::StartGame(g) => {
                        if let Some(gid) = g {
                            t.send(Event::Net(Comm::StartGame(g)));
                        }
                    }, 
                }
            }
        });

        Network(o)
    }

    pub fn send (&mut self,c:Comm) {
        self.0.send(&c);
    }
}
