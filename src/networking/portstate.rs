pub struct PortState{
    pub open: bool,
}
impl Default for PortState{
    fn default() -> Self {
        PortState{
            open: false
        }
    }
}
pub trait State {
    fn is_open(&self) -> bool;
    fn is_closed(&self) -> bool;
    fn open(&mut self);
    fn closed(&mut self);
}
impl State for PortState{
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
}
