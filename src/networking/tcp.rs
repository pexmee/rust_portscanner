use eyre;
use futures::future;
use log::{debug, info, warn};
use std::io;
use std::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str;
use std::time::Duration;
use tokio::net::TcpStream as TcpStreamAsync;
use super::ports::{Port, State};

const ALLOWED_CON_ERRORS: &[io::ErrorKind] = &[
    io::ErrorKind::ConnectionRefused,
    io::ErrorKind::ConnectionAborted,
    io::ErrorKind::ConnectionReset,
];

pub fn tcp_connect<'a>(host: &'a str, proto: &'a str, port: &mut Port) -> eyre::Result<(), io::Error> {
    let ip = host.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port.into());
    let port_result = TcpStream::connect_timeout(&addr, Duration::new(0, 1_000_000_000));
    match port_result {
        Err(e) if ALLOWED_CON_ERRORS.contains(&e.kind()) => {
            port.open = false;
            debug!("{} port: {} state: {}", proto, port.value, port.state());
            Ok(())
        }
        Ok(_) => {
            port.open = true;
            debug!("{} port: {} state: {}", proto, port.value, port.state());
            Ok(())
        }

        Err(e) => {
            warn!("Scanning port: {} returned error: {}", port.value, e);
            Err(e)
        }
    }
}

pub async fn tcp_connect_async(
    host: String,
    proto: String,
    mut port: Port,
) -> eyre::Result<(), io::Error> {
    let ip = host.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port.into());
    let port_result = TcpStreamAsync::connect(addr).await;
    match port_result {
        Err(e) if ALLOWED_CON_ERRORS.contains(&e.kind()) => {
            port.open = false;
            debug!("{} port: {} state: {}", proto, port.value, port.state());
            Ok(())
        },
        Ok(_) => {
            port.open = true;
            info!("{} port: {} state: {}", proto, port.value, port.state());
            Ok(())
        },
        Err(e) => {
            warn!("Scanning port: {} returned error: {}", port.value, e);
            Err(e)
        }
    }
}

pub async fn scan_ports(
    hostname: String,
    proto: String,
    mut ports: Vec<Port>,
) -> eyre::Result<Vec<Port>, io::Error> {
    let mut futures = vec![];
    for port in ports.iter_mut() {
        if !port.is_seen() {
            port.seen = true;
            let future = tokio::task::spawn(tcp_connect_async(
                hostname.to_string(),
                proto.to_string(),
                *port,
            ));
            futures.push(future);
        }
    }
    for result in future::join_all(futures).await {
        match result.unwrap() {
            Err(e) => {
                return Err(e)
            },
            _ => (),
        }
    }
    Ok(ports)
}
