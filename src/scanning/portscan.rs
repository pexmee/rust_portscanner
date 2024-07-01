use std::net::TcpStream;
use std::time::Duration;
use ::tokio;
use eyre;
use futures::future;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpStream as TcpStreamAsync;
const ALLOWED_CON_ERRORS: &[io::ErrorKind] = &[
    io::ErrorKind::ConnectionRefused,
    io::ErrorKind::ConnectionAborted,
    io::ErrorKind::ConnectionReset,
];

pub struct Target {
    hostname: String,
    proto: String, // Not used yet because we don't have support for it
    start_port: u16,
    end_port: u16,
}

pub fn create_target(hostname: String, proto: String, start_port: u16, end_port: u16) -> Target{
    Target{
        hostname: hostname,
        proto: proto,
        start_port: start_port,
        end_port: end_port,
    }
}


/// TCP connect returning port and state or err
pub fn inspect_port(hostname: String, port: u16) -> eyre::Result<(u16, bool), io::Error> {
    let ip = hostname.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port.into());
    let port_result = TcpStream::connect_timeout(&addr, Duration::new(0, 1_000_000_000));
    match port_result {
        Err(e) if ALLOWED_CON_ERRORS.contains(&e.kind()) => {
            debug!("TCP port: {} state: CLOSED", port);
            Ok((port, false))
        }
        Ok(_) => {
            info!("TCP port: {} state: OPEN", port);
            Ok((port, true))
        }
        Err(e) => {
            warn!("Scanning TCP port: {} returned error: {}", port, e);
            Err(e)
        }
    }
}
/// Asynchronous TCP connect returning port and state or err
pub async fn inspect_port_async(hostname: String, port: u16) -> eyre::Result<(u16, bool), io::Error> {
    let ip = hostname.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port);
    let port_result = TcpStreamAsync::connect(addr).await;
    match port_result {
        Err(e) if ALLOWED_CON_ERRORS.contains(&e.kind()) => {
            debug!("TCP port: {} state: CLOSED", port);
            Ok((port, false))
        }
        Ok(_) => {
            info!("TCP port: {} state: OPEN", port);
            Ok((port, true))
        }
        Err(e) => {
            warn!("Scanning TCP port: {} returned error: {}", port, e);
            Err(e)
        }
    }
}

pub async fn scan_target(
    target: Target,
    port_map: &mut HashMap<u16, bool>,
) -> eyre::Result<&HashMap<u16, bool>, io::Error> {
    let mut futures = Vec::with_capacity(target.end_port.into());
    for port in target.start_port..target.end_port {
        let future = tokio::task::spawn(inspect_port_async(target.hostname.to_string(), port));
        futures.push(future);
    }
    for result in future::join_all(futures).await {
        match result.unwrap() {
            Err(e) => return Err(e),
            Ok(r) => {
                let (port, open) = r;
                port_map.entry(port).or_insert(open);
            }
        }
    }
    Ok(port_map)
}