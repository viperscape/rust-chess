extern crate wire;
extern crate cubby;
extern crate rand;

use super::{Position,Move,Event,Player,Item};
use std::thread;
use self::wire::{SizeLimit,tcp};

use std::sync::mpsc::{Sender};

use std::sync::{Arc};
use self::cubby::{mutex,rwlock,Eid};

#[derive(Debug,RustcDecodable, RustcEncodable,Copy)]
pub enum Comm {
    Move(Position,Position), //from, to; todo: consider using lib's move
    StartGame(Option<u64>),
    EndGame, //game officially over
    //Chat(String),
    Quit, //network is shutdown
}

struct Players(mutex::CubbyMutex<tcp::OutTcpStream<Comm>>);
impl Players {
    fn new () -> Players {
        Players(mutex::build(4000))
    }
}

struct NetGame {
    white: Option<Eid>,
    black: Option<Eid>,
    moves: Vec<Move>,
    id: u64, //consider removing? 
}

struct Games(rwlock::CubbyRwLock<NetGame>);
impl Games {
    fn new () -> Games {
        Games(rwlock::build(2000))
    }

    fn insert (&self) -> Eid {
        self.0.add(NetGame { white: None,
                             black: None,
                             moves: vec!(), 
                             id: rand::random::<u64>()}).unwrap()
    }

    // todo: check for what side player should be on!
    /// currently returns the side they are on, and the last move played in the game
    fn attach (&self, e: Eid, p:Eid) -> (Option<Player>,Option<Move>) {
        let mut last_move = None;
        let r = self.0.with_mut(e, |g| {
            last_move = g.moves.clone().pop();

            if g.white.is_some() {
                g.black = Some(p);
                Some(Player::Black(Item::Pawn))
            }
            else {
                g.white = Some(p);
                Some(Player::White(Item::Pawn))
            }

        });

        match r {
            Ok(rr) => (rr,last_move),
            _ => (None, last_move),
        }
    }

    fn update (&self, gid:Eid, m: (Position,Position), pid: Eid) -> Option<Eid> {
        let r = self.0.with_mut(gid, |g| {
            g.moves.push(m);
            if g.white.is_some() && g.black.is_some() {
                if g.white.unwrap() == pid { g.black }
                else { g.white }
            }
            else { None }
        });

        match r {
            Ok(rr) => rr,
            _ => None,
        }
        
    }

    fn find_game (&self, id:u64) -> Option<Eid> {
        self.0.first(|g| g.id == id)
    }
}

pub struct Network(tcp::OutTcpStream<Comm>);
impl Network {
    pub fn new_server (){

        let (listener,_) = wire::listen_tcp(("localhost", 9999)).unwrap();

        let games = Arc::new(Games::new());
        let players = Arc::new(Players::new());

        
        for conn in listener.into_blocking_iter() {
            let _games = games.clone();
            let _players = players.clone();

            thread::spawn(move || {
                let (i,o) = wire::upgrade_tcp((conn.0),
                                          SizeLimit::Bounded(1000),
                                          SizeLimit::Bounded(1000)).unwrap();

                let mut gid: Option<Eid> = None;
                let pid = _players.0.add(o).unwrap();

                for n in i.into_blocking_iter() {
                    match n {
                        Comm::Move(f,t) => {
                            if gid.is_some() {
                                let other = _games.update(gid.unwrap(),(f,t), pid);
                                if let Some(_other) = other { //pass along new move
                                    _players.0.with_mut(_other,|p| p.send(&n));
                                }
                            }
                            else { break; } // todo: nice-disconnect
                        },
                        Comm::StartGame(_g) => {
                            if let Some(_gid) = _g {
                                if let Some(_eid) = _games.find_game(_gid) {
                                    gid = Some(_eid);
                                    _games.attach(_eid,pid);
                                }
                            }
                            else { //generate a game id
                                gid = Some(_games.insert());
                            } 
                        }, 
                        Comm::EndGame => { //end current game
                            if gid.is_some() {
                                _games.0.with(gid.unwrap(),|g| {
                                    if let Some(_p) = g.white {
                                        _players.0.with_mut(_p,|p| p.send(&n));
                                    }
                                    if let Some(_p) = g.black {
                                        _players.0.with_mut(_p,|p| p.send(&n));
                                    }
                                });
                                _games.0.remove(gid.unwrap());
                            }
                            break;
                        },
                        Comm::Quit => {
                            break;
                        },
                        //_ => (),
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

        thread::spawn(move || {
            for n in i.into_blocking_iter() {
                t.send(Event::Net(n));
            }
            t.send(Event::Net(Comm::Quit));
        });

        Network(o)
    }

    pub fn send_server (&mut self,c:Comm) {
        self.0.send(&c);
    }

    //fn send_client(&mut self, c:Comm, pid:Eid) {    
   // }
}
