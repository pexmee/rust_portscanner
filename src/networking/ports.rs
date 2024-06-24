#[derive(Clone)]
pub struct Port {
    pub value: u16,
    open: bool,
    seen: bool,
}


impl Into<u16> for Port {
    fn into(self) -> u16 {
        self.value
    }
}
impl From<u16> for Port {
    fn from(port: u16) -> Self {
        Port {
            value: port,
            open: false,
            seen: false,
        }
    }
}
impl From<&mut Port> for u16 {
    fn from(port: &mut Port) -> Self {
        port.value
    }
}

impl From<&Port> for u16 {
    fn from(port: &Port) -> Self {
        port.value
    }   
}
pub trait State {
    fn is_open(&self) -> bool;
    fn is_closed(&self) -> bool;
    fn open(&mut self);
    fn closed(&mut self);
    fn seen(&self) -> bool;
    fn see(&mut self);
}
impl State for Port {
    fn is_open(&self) -> bool {
        self.open == true
    }
    fn is_closed(&self) -> bool {
        self.open == false
    }
    fn open(&mut self) {
        self.open = true;
    }
    fn closed(&mut self) {
        self.open = false;
    }
    fn seen(&self) -> bool {
        self.seen
    }
    fn see(&mut self) {
        self.seen = true
    }
}
