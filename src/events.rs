use std::sync::mpsc::{Receiver,Sender,channel};
use super::{Inputs,Comm,Render};


pub struct Events {
    t: Sender<Event>,
    r: Receiver<Event>,
}

impl Events {
    pub fn new () -> Events {
        let (t,r) = channel();
        Events{ t:t, r:r }
    }

    pub fn branch (&self) -> Sender<Event> {
        self.t.clone()
    }
}

impl Iterator for Events {
    type Item = Event;
    fn next (&mut self) -> Option<Event> {
        match self.r.recv() {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Net(Comm),
    Inp(Inputs),
    Gfx(Render),
    //Quit, //generic quit event
}
