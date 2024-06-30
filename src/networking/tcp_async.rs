use eyre;
use log::debug;
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
// use std::str;
use super::ports::{Port, State};
use crate::networking::tcp::ConnectionError;
use tokio::net::TcpStream;

/*
TODO: Perhaps make this take a port list instead or something.
*/ 
pub async fn tcp_connect_async(
    host: String,
    mut port: Port,
) -> eyre::Result<Port, ConnectionError> {
    let ip = host.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port.into());
    let port_result = TcpStream::connect(addr).await;
    match port_result {
        Err(ref e)
            if [
                ErrorKind::PermissionDenied,
                ErrorKind::NotFound,
                ErrorKind::NotConnected,
            ]
            .contains(&e.kind()) =>
        {
            debug!("port: {} returned err: {}", port.value, e);
            // Err(ConnectionError::ConnectionError())
            panic!("connection error, sure target is alive?");

        }
        Err(ref e) if e.kind() == ErrorKind::TimedOut => Err(ConnectionError::ConnectionTimeout()),
        Err(ref e) => {
            debug!("port: {} returned err: {}", port.value, e);
            Ok(port)
        }

        Ok(_) => {
            port.open();
            Ok(port)
        }
    }
}
