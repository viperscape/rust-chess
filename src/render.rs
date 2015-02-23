use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use super::{Event};

#[derive(Debug)]
pub enum Render {
    Pause, //rename to halt?
    Step,
    Resize(u32,u32),
    Quit,
}

impl Render {
    pub fn new (t: Sender<Event>) -> Sender<Event> {
        let (_tx,r) = channel();

        thread::spawn(move || {
            for e in r.iter() {
                match e {
                    Event::Gfx(re) => {
                        match re {
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }

            t.send(Event::Gfx(Render::Quit));
        });

        _tx
    }
}
