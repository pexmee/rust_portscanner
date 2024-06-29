// use std::io::{prelude::*};
use eyre;
use log::debug;
use std::io::ErrorKind;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str;
use std::time::Duration;

use super::ports::Port;
pub enum ConnectonState {
    Open(),
    Closed(),
}
#[derive(Debug)]
pub enum ConnectionError {
    ConnectionError(),
    ConnectionTimeout(),
}

pub fn tcp_connect<'a>(host: &'a str, port: &Port) -> eyre::Result<ConnectonState, ConnectionError> {
    let ip = host.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port.into());
    let port_result = TcpStream::connect_timeout(&addr, Duration::new(0, 1_000_000_000));
    match port_result {
        // Using list comprehension if we want to add errors
        Err(ref e) if [ErrorKind::PermissionDenied, ErrorKind::NotFound, ErrorKind::NotConnected].contains(&e.kind()) => {
            debug!("port: {} returned err: {}", port.value, e);
            Err(ConnectionError::ConnectionError())
        }
        Err(ref e) if e.kind() == ErrorKind::TimedOut => {
            Err(ConnectionError::ConnectionTimeout())
        }
        Err(ref e) => {
            debug!("port: {} returned err: {}", port.value, e);
            Ok(ConnectonState::Closed())},

        Ok(_) => Ok(ConnectonState::Open()),
    }
}