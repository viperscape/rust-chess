use std::sync::mpsc::{Sender,Receiver};
use std::thread;
use super::{Event,Render};
use std::cell::RefCell;


extern crate piston;
use self::piston::input::{Button, MouseButton};
use self::piston::input::keyboard::Key;
use self::piston::event::{
    PressEvent,
    ReleaseEvent,
    MouseCursorEvent,
    MouseScrollEvent,
    MouseRelativeEvent,
    TextEvent,
    ResizeEvent,
    FocusEvent,
    RenderEvent,
    UpdateEvent
};

extern crate glutin_window;
use self::glutin_window::GlutinWindow as Window;


#[derive(Debug)]
pub enum Inputs {
    Keyboard(Key),
    Mouse(MouseButton, (f64,f64)),
    Quit
}

impl Inputs {
    pub fn new (window: RefCell<Window>, render: Sender<Event>, t: Sender<Event>) {
        thread::spawn(move || {
            let mut mpos = None; //piston events are wiped on each event it seems, so store this outside the loop
            for e in piston::events(&window) {
                e.mouse_cursor(|x, y| { mpos = Some((x,y)); }); 

                if let Some(Button::Mouse(btn)) = e.release_args() {
                    if let Some(xy) = mpos {
                        t.send(Event::Inp(Inputs::Mouse(btn, xy)));
                    }
                }
                if let Some(Button::Keyboard(key)) = e.release_args() {
                    t.send(Event::Inp(Inputs::Keyboard(key)));
                };

                e.resize(|w, h| render.send(Event::Gfx(Render::Resize(w,h))));
                if let Some(focused) = e.focus_args() {
                    if focused { render.send(Event::Gfx(Render::Step)); }
                    else { render.send(Event::Gfx(Render::Pause)); }
                };

                e.render(|_| {});
                e.update(|_| {});
            }

            t.send(Event::Inp(Inputs::Quit));

        });
    }
}
