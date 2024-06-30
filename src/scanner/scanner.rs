use std::io;

// clippy
use crate::networking::ports::Port;
use crate::networking::tcp::{tcp_connect, scan_ports};
// use eyre;
use log::{info, warn};
use rand::seq::SliceRandom;
/// This is a docstring
/// ```rust
/// print!("hello");
/// ```
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
fn target_responsive(hostname: &str, proto: &str, port: &mut Port) -> bool {
    let result = tcp_connect(hostname, proto, port);
    match result{
        Err(e) => {
            warn!("Sure the target is up and running? Port scan returned with error: {}", e);
            return false;
        }
        Ok(_) => {
            return true;
        }
    }
}
pub trait Scan {
    fn target_alive(&mut self, inspected_ports: bool) -> bool;
    async fn scan(&mut self) -> eyre::Result<(), io::Error>; 
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
            for port in self.ports.iter_mut() {
                if count >= 5 {
                    return false;
                }
                info!("scanning port: {}", port.value);
                if target_responsive(&self.hostname, &self.proto, port) {
                    return true;
                }
            }
        }
        // check known ports
        else {
            info!("Checking if target is alive with prior inspected ports");
            let mut ports = self.ports.to_vec();
            ports.shuffle(&mut rand::thread_rng());
            for port in ports.iter_mut() {
                info!("scanning port: {}", port.value);
                if self.ports.len() > 20 && count >= 20 {
                    // Unlikely that the target is alive
                    return false;
                }
                if target_responsive(&self.hostname, &self.proto, port) {
                    return true;
                }
                count += 1;
            }
        }

        false
    }
    async fn scan(&mut self) -> eyre::Result<(), io::Error> {
        // here we gotta do some multithreading bs to scan ports in different segments
        // if !self.target_alive(false) {
        //     info!(
        //         "The target is not responding on provided ports. Are you sure the target is alive?"
        //     );
        //     return Ok(())
        // }
        // info!("target alive");
        // Here we do multithreading
        // self.inspect_ports();
        match scan_ports(self.hostname.to_string(), self.proto.to_string(), self.ports.clone()).await{
            Ok(ports) => {
                self.ports = ports.clone();
                return Ok(());
            },
            Err(e) => {
                Err(e)
            },
        }
    }
}

