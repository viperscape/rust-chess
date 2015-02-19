extern crate wire;
extern crate cubby;

use super::{Game,Position,Event,Player,Item};
use std::thread::Thread;
use self::wire::{SizeLimit,tcp};
use std::rand;

use std::sync::mpsc::{channel,Receiver,Sender};

//use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use self::cubby::{Ent,Eid, EntErr};

#[derive(Debug,RustcDecodable, RustcEncodable,Copy)]
pub enum Comm {
    Move(Position,Position), //from, to
    StartGame(Option<u64>),
    EndGame,
    //Chat(String),
}

struct Players(Ent<tcp::OutTcpStream<Comm>>);
impl Players {
    fn new () -> Players {
        Players(Ent::new(2000))
    }
}

struct NetGame {
    white: Option<Eid>,
    black: Option<Eid>,
    moves: Vec<(Position,Position)>,
    id: u64, //consider removing? 
}

struct Games(Ent<NetGame>);
impl Games {
    fn new () -> Games {
        Games(Ent::new(2000))
    }

    fn insert (&self) -> Eid {
        self.0.add(NetGame { white: None,
                             black: None,
                             moves: vec!(), 
                             id: rand::random::<u64>()}).unwrap()
    }

    // todo: check for what side player should be on!
    fn attach (&self, e: Eid, p:Eid) -> Option<Player> {
        let r = self.0.with_mut(e, move |g| {
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
            Ok(rr) => rr,
            _ => None,
        }
    }

    fn update (&self, e:Eid, m: (Position,Position)) {
        self.0.with_mut(e, |g| {
            g.moves.push(m)
        });
    }

    fn find_game (&self, id:u64) -> Option<Eid> {
        self.0.find(|g| { if g.id == id {Some(EntErr::Break)}
                          else {None} })
    }
}

pub struct Network(tcp::OutTcpStream<Comm>);
impl Network {
    pub fn new_server (){

        let (listener,_) = wire::listen_tcp(("localhost", 9999)).unwrap();

        let games = Arc::new(Games::new());
        let players = Arc::new(Players::new());

        
//mut o:tcp::OutTcpStream<Comm>
        for conn in listener.into_blocking_iter() {
            let _games = games.clone();
            let _players = players.clone();

            Thread::spawn(move || {
                let (i, mut o) = wire::upgrade_tcp(conn,
                                                   SizeLimit::Bounded(1000),
                                                   SizeLimit::Bounded(1000));

                let mut gid: Option<Eid> = None;
                let pid = _players.0.add(o).unwrap();

                for n in i.into_blocking_iter() {
                    match n {
                        Comm::Move(f,t) => {
                            if gid.is_some() {
                                
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
                                _games.0.remove(gid.unwrap());
                            }
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
                    Comm::EndGame => {
                        t.send(Event::Net(Comm::EndGame));
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
