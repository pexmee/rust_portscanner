use eyre;
use log::debug;
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str;
use crate::networking::tcp::ConnectionError;
use crate::networking::tcp::ConnectonState;
use tokio::net::TcpStream;
use super::ports::Port;

pub async fn tcp_connect_async<'a>(host: &'a str, port: &Port) -> eyre::Result<ConnectonState, ConnectionError>{
    let ip = host.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port.into());
    let port_result = TcpStream::connect(addr).await;
    match port_result{
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