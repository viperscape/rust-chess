extern crate wire;

use super::{Game,Position,Event};
use std::thread::Thread;
use self::wire::{SizeLimit,tcp};
use std::rand;

use std::sync::mpsc::{channel,Receiver,Sender};


#[derive(Debug,RustcDecodable, RustcEncodable,Copy)]
pub enum Comm {
    Move(Position,Position), //from, to
    StartGame(Option<u64>),
    EndGame(u64),
}

// all open games
pub struct Network;
impl Network {
    pub fn new_server () {

        let (listener, _) = wire::listen_tcp(("localhost", 9999)).unwrap();

        for conn in listener.into_blocking_iter() {
            Thread::spawn(move || {
                let (i, mut o) = wire::upgrade_tcp(conn,
                                                   SizeLimit::Bounded(8),
                                                   SizeLimit::Bounded(8));

                for n in i.into_blocking_iter() {
                    match n {
                        Comm::Move(f,t) => (),
                        Comm::StartGame(g) => {
                            if let Some(gid) = g {
                                o.send(&Comm::StartGame(g));
                            }
                            else { o.send(&Comm::StartGame(Some(rand::random::<u64>()))); } //generate a game id
                        }, 
                        Comm::EndGame(gid) => {
                            break;
                        }
                    }
                    
                }
            });
        }
    }


    pub fn new_client (gid: Option<u64>, t: Sender<Event>) {
        let (i, mut o) = wire::connect_tcp(("localhost",9999),
                                           SizeLimit::Bounded(8),
                                           SizeLimit::Bounded(8)).unwrap();

        Thread::spawn(move || {
            o.send(&Comm::StartGame(gid));

            for n in i.into_blocking_iter() {
                match n {
                    Comm::Move(f,to) => { t.send(Event::Net(Comm::Move(f,to))); },
                    Comm::EndGame(gid) => {
                        t.send(Event::Net(Comm::EndGame(gid)));
                    },
                    Comm::StartGame(g) => {
                        if let Some(gid) = g {
                            t.send(Event::Net(Comm::StartGame(g)));
                        }
                    }, 
                }
            }
        });
    }
}
