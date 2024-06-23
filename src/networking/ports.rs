

#[derive(Clone)]
pub struct Port{
    pub port: u16,
    open: bool,
    seen: bool,
}


pub fn create_port(port: u16) -> Port {
    Port{
        port: port,
        open: false,
        seen: false
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
impl State for Port{
    fn is_open(&self) -> bool{
        self.open == true
    }
    fn is_closed(&self) -> bool{
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
    fn see(&mut self){
        self.seen = true
    }
}