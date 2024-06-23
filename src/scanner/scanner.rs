use std::str;
use std::vec;

use crate::networking::ports::State;
use crate::networking::ports::{Port, create_port};
use crate::networking::tcp::tcp_connect;
use rand::seq::SliceRandom;

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
fn is_alive(hostname: &str, port: u16) -> bool{
    let result = tcp_connect(hostname, port);
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}
pub trait Scan {
    fn target_alive(&self) -> bool;
    fn scan(&mut self);
}

impl<'t> Scan for Scanner<'t> {
    fn target_alive(&self) -> bool {
        /*
        Responsible for checking if the target is still up and running.
        It checks random seen ports again to see if it responds.
        If more than 3 failed attempts occur on known ports, we consider
        the target down.
        */
 
        let mut count = 0;
        let mut ports = self.ports.to_vec();
        ports.shuffle(&mut rand::thread_rng());
        for port in ports.iter(){
            if count >= 3{
                return false
            }
            if port.seen(){
                if is_alive(&self.hostname, port.port){
                    return true;
                }
                count += 1;
            }

        }
        false
    }
    fn scan(&mut self) {
        // here we gotta do some multithreading bs to scan ports in different segments
        for port in self.ports.iter_mut(){
            if !port.seen(){
                let scan_result = tcp_connect(self.hostname, port.port);
                match scan_result{
                    Ok(_) => {
                        port.open();
                    },
                    Err(_) => {
                        port.closed(); // This should technically not do anything as it defaults to closed anyway
                    }
                }
                port.see();
            }
        }
    }
}
