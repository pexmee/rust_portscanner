use std::str;
use std::vec;

use crate::networking::ports::State;
use crate::networking::ports::{Port, create_port};

#[derive(Default)]
pub struct Scanner<'t> {
    hostname: &'t str,
    proto: &'t str,
    port_range_str: &'t str,
    ports: Vec<Port>,
}
pub fn create_scanner<'t>(hostname: &'t str, proto: &'t str, port_range_str: &'t str) -> Scanner<'t>{
    let mut scanner = Scanner{
        hostname: hostname,
        proto: proto,
        port_range_str: port_range_str,
        ports: Vec::new(),
    };

    // Initialize the ports
    let mut start_port = 1;
    let mut end_port = 65535;

    // If we have a user supplied port range
    if !scanner.port_range_str.is_empty() {
        let port_range: Vec<&str> = scanner.port_range_str.split("-").collect();
        start_port = port_range[0].parse().unwrap();
        end_port = port_range[1].parse().unwrap();
    }
    for port in start_port..end_port as u16{
        scanner.ports.push(create_port(port));
    }
    // debug stuff
    // for port in scanner.ports.iter(){
    //     println!("port:{}, open:{}, seen:{}", port.port, port.is_open(), port.seen());
    // }
    scanner

}
pub trait Scan {
    fn scan(&mut self);
}

impl<'t> Scan for Scanner<'t> {
    fn scan(&mut self) {
    }
}
