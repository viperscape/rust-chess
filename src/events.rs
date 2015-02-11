use std::sync::mpsc::{Receiver};
use super::{Inputs,Comm};

pub struct Events {
    inp: Receiver<Inputs>,
    net: Receiver<Comm>,
}

impl Events {
    pub fn new (inp: Receiver<Inputs>, net: Receiver<Comm>,) -> Events {
        Events {inp:inp,net:net}
    }
    
    pub fn with<F,F2> (&self, mut fnet: F, mut finp: F2)
        where F: FnMut(Comm),F2: FnMut(Inputs) {

            let net = &self.net;
            let inp = &self.inp;

            select! (comm = net.recv() => fnet(comm.unwrap()),
                     act = inp.recv() => finp(act.unwrap()));
        }
}
