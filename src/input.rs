use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread::Thread;

// ignore this, will be replaced by piston stuff
// callbacks will likely be passed in before inputs is created
// as in: an input handler
pub enum Input {
    Key(u16),
    Mouse1,
}

pub struct Inputs;
impl Inputs {
    pub fn new () -> Receiver<Input> {
        let (t,r) = channel();

        Thread::spawn(move || {
            t.send(Input::Mouse1);
        });

        r
    }
}
