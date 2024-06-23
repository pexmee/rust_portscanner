use std::str;
use std::vec;
#[derive(Default)]
pub struct Scanner<'t> {
    pub hostname: &'t str,
    pub proto: &'t str,
    pub port_range_str: &'t str,
    pub port_range: Option<[u32; 2]>,
}

pub trait Scan {
    fn get_port_range(&mut self);
}

impl<'t> Scan for Scanner<'t> {
    fn get_port_range(&mut self) {
        if self.port_range_str.is_empty() {
            self.port_range = Some([1, 65535]);
        } else {
            let port_range: Vec<&str> = self.port_range_str.split("-").collect();
            self.port_range = Some([
                port_range[0].parse().unwrap(),
                port_range[1].parse().unwrap(),
            ])
        }
    }
}
