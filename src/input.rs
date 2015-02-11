use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread::Thread;
use super::{Event};

// ignore this, will be replaced by piston stuff
// callbacks will likely be passed in before inputs is created
// as in: an input handler
#[derive(Debug)]
pub enum Inputs {
    Key(u16),
    Mouse1,
}

impl Inputs {
    pub fn new (t: Sender<Event>) {
        Thread::spawn(move || {
            t.send(Event::Inp(Inputs::Mouse1));
        });
    }
}
