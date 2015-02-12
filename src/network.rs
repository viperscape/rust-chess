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
    EndGame,
    //Chat(String),
}

struct NetGame {
    white: u64, // 0 denotes none
    black: u64,
    moves: Vec<(Position,Position)>,
    id: u64,
}


// connected players
struct Players(HashMap<u64,tcp::OutTcpStream<Comm>>);
impl Players {
    fn new () -> Players {
        Players(HashMap::new())
    }

    fn attach (&mut self, o:tcp::OutTcpStream<Comm>) -> u64 {
        let id = rand::random::<u64>();
        self.0.insert(id,o);
        id
    }

    fn detach (&mut self, id:u64) {
        self.0.remove(id);
    }

    fn send (&mut self, id:u64, c:Comm) -> Result<> {
        self.0.get_mut(id).send(&c)
    }
}


// all open games
struct Games(HashMap<u64,NetGame>);
impl Games {
    fn new () -> Games {
        Games(HashMap::new())
    }

    fn insert (&mut self, id:u64) {
        self.0.insert(id,NetGame { white:0,
                                   black:0,
                                   moves:vec!(),
                                   id:id });
    }

    // todo: check for what side player should be on!
    fn attach (&mut self, gid:u64, pid:u64) -> Option<Player> {
        match self.0.get_mut(&gid) {
            Some(g) => {
                if g.white.is_some() {
                    g.black = pid;
                    Some(Player::Black(Item::Pawn))
                }
                else {
                    g.white = pid;
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
        let players = Arc::new(Mutex::new(Players::new()));

        for conn in listener.into_blocking_iter() {
            let _games = games.clone();
            let _players = players.clone();

            Thread::spawn(move || {
                let (i, mut o) = wire::upgrade_tcp(conn,
                                                   SizeLimit::Bounded(1000),
                                                   SizeLimit::Bounded(1000));

                let mut gid:u64 = 0;
                let mut pid:u64 = 0;

                for n in i.into_blocking_iter() {
                    match n {
                        Comm::Move(f,t) => {
                            
                        },
                        Comm::StartGame(g) => {
                            if let Some(_gid) = g {
                                gid = _gid;

                                // add player
                                let pid = {
                                    let mut pl = _players.lock().unwrap();
                                    *pl.attach(o)
                                };

                               /* //lookup game id
                                let r = {
                                    let mut gl = _games.lock().unwrap();
                                    *gl.attach(gid)
                                };*/

                                with_games(_games,|g| {
                                    g.attach(gid)
                                });

                                with_players(_players,|p| {
                                    p.send(&Comm::StartGame(g));
                                });
                                
                            }
                            else { //generate a game id
                                let ng = rand::random::<u64>();
                                with_players(_players,|p| {
                                    p.send(&Comm::StartGame(Some(ng)));
                                });
                                gid = Some(ng);
                            } 
                        }, 
                        Comm::EndGame => { //end current game
                            with_players(_players,|p| {
                                p.send(&Comm::EndGame);
                            });
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


fn with_players (p: Arc<Mutex<Players>>, f: F) where F: FnMut(Players) {
    let r = {
        let mut pl = p.lock().unwrap();
        f(*pl)
    };
}

fn with_games (g: Arc<Mutex<Games>>, f: F) where F: FnMut(Games) {
    let r = {
        let mut gl = g.lock().unwrap();
        f(*gl)
    };
}
