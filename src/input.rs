use std::sync::mpsc::{Sender,Receiver};
use std::thread;
use super::{Event,Render};

extern crate glutin;
//use glutin::{ElementState,VirtualKeyCode,MouseButton};

#[derive(Debug)]
pub enum Inputs {
    Key(glutin::VirtualKeyCode),
    Click(glutin::MouseButton, (i32,i32)),
    Drag((i32,i32)),
    Quit
}

/*pub trait Input<T,U> {
    fn new (inpr: Receiver<T>, t: Sender<U>);
}*/

/// translates and collects glutin inputs
/// glutin polling of events can glom events from other pollings
/// so we send events to this thread, instead of cloning displays to poll for
/// the output (t: Sender) is for a receiving game-loop
impl Inputs {
    pub fn new (inpr: Receiver<glutin::Event>, t: Sender<Event>) {
        thread::spawn(move || {
            let mut mpos: (i32,i32) = (0,0); 
            for e in inpr.iter() {
                match e {
                    glutin::Event::MouseMoved(pos) => { mpos = pos; },
                    glutin::Event::MouseInput(state, btn) => {
                        match state {
                            glutin::ElementState::Released => { 
                                t.send(Event::Inp(Inputs::Click(btn,mpos))); 
                            },
                            _ => (),
                        }
                    },
                    glutin::Event::KeyboardInput(state,_,vkey) => {
                        match state {
                            glutin::ElementState::Released => {
                                if let Some(key) = vkey {
                                    t.send(Event::Inp(Inputs::Key(key)));
                                }
                            },
                            _ => (),
                        }
                    },
                    glutin::Event::Closed => break,
                    _ => (),
                }
            }

            t.send(Event::Inp(Inputs::Quit)); //tell main thread we're done
        });
    }
}
