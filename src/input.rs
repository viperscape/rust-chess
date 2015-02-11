use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread::Thread;

// ignore this, will be replaced by piston stuff
// callbacks will likely be passed in before inputs is created
// as in: an input handler
pub enum Inputs {
    Key(u16),
    Mouse1,
}

impl Inputs {
    pub fn new () -> Receiver<Inputs> {
        let (t,r) = channel();

        Thread::spawn(move || {
            t.send(Inputs::Mouse1);
        });

        r
    }
}
