

// clippy
use crate::networking::ports::Port;
use crate::networking::ports::State;
use crate::networking::tcp::tcp_connect;
use crate::networking::tcp::ConnectionError;
use crate::networking::tcp::ConnectonState;

use log::debug;
use log::info;
use rand::seq::SliceRandom;

pub struct Scanner<'t> {
    hostname: &'t str,
    proto: &'t str, // Not used yet because we don't have support for it
    ports: Vec<Port>,
    start_port: u16,
    end_port: u16,
}

pub fn create_scanner<'t>(
    hostname: &'t str,
    proto: &'t str,
    start_port: u16,
    end_port: u16,
) -> Scanner<'t> {
    let mut scanner = Scanner {
        hostname: hostname,
        proto: proto,
        start_port: start_port,
        end_port: end_port,
        ports: Vec::new(),
    };
    // This defaults to 1-65535, see port_parser
    for port in scanner.start_port..scanner.end_port as u16 {
        scanner.ports.push(Port::from(port));
    }
    scanner
}
fn target_responsive(hostname: &str, port: &Port) -> Result<bool,ConnectionError> {
    let result = tcp_connect(hostname, port);
    match result {
        Err(ConnectionError::ConnectionTimeout()) => {
            Err(ConnectionError::ConnectionTimeout())
        },
        Err(_) =>{
            info!("Got connection error from target. Sure the target is alive?");
            Ok(false)
        }
        Ok(_) => {
            Ok(true)
        },
    }
}
pub trait Scan {
    fn target_alive(&mut self, inspected_ports: bool) -> bool;
    fn scan(&mut self);
}

trait Inspect {
    fn inspect_ports(&mut self);
}

impl<'t> Scan for Scanner<'t> {
    fn target_alive(&mut self, inspected_ports: bool) -> bool {
        /*
        Responsible for checking if the target is still up and running.
        It checks random seen ports again to see if the target responds.
        If more than 5 failed attempts occur on known ports, we consider
        the target down.
        If we do have seen ports, it should continue as normal, i.e read the description.
        */
        let mut count = 0;
        if !inspected_ports {
            info!("Checking if target is alive with no prior inspected ports");
            for port in self.ports.iter() {
                if count >= 5{
                    return false
                }
                info!("scanning port: {}", port.value);
                match target_responsive(&self.hostname, port) {
                    Ok(v) => return v,
                    Err(e) => {
                        debug!("connection timed out {:?}", e);
                        count +=1
                    }
                }
            }
        }
        // check known ports
        else {
            info!("Checking if target is alive with prior inspected ports");
            let mut ports = self.ports.to_vec();
            ports.shuffle(&mut rand::thread_rng());
            for port in ports.iter() {
                info!("scanning port: {}", port.value);
                if ports.len() > 20 && count >= 20 {
                    // Unlikely that the target is alive
                    return false;
                }
                match target_responsive(&self.hostname, port) {
                    Ok(v) => {
                        return v
                    }
                    Err(e) => {
                        debug!("connection timed out {:?}", e);
                        count +=1
                    }
                }
                count += 1;
            }
        }

        false
    }
    fn scan(&mut self){
        // here we gotta do some multithreading bs to scan ports in different segments
        if !self.target_alive(false) {
            info!("The target is not responding on provided ports. Are you sure the target is alive?");
            return
        }
        info!("target alive");
        // Here we do multithreading
        self.inspect_ports();
    }
}

impl<'t> Inspect for Scanner<'t>{
    fn inspect_ports(&mut self){
        let mut count = 0;
        for port in self.ports.iter_mut() {
            if !port.seen() {
                port.see();
                match tcp_connect(self.hostname, port){
                    Err(ConnectionError::ConnectionTimeout()) => {
                        count += 1;
                        if count >= 5{
                            info!("Connection timed out {} times on different ports. Sure the target is alive?", count);
                            return
                        }
                    }
                    Err(_) =>{
                        panic!("Got connection error")
                    }
                    Ok(ok) => {
                        match ok {
                            ConnectonState::Open() => {
                                port.open();
                            }
                            ConnectonState::Closed() => {
                                port.closed();
                            }
                        };
                        info!("port:{}, proto:{}, state:{}", port.value, self.proto, port.state());
                        count = 0;
                    }
                };
            }
        }
    }
}