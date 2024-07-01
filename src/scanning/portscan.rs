use ::tokio;
use eyre;
use futures::future;
use log::{debug, info};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::io;
use tokio::net::TcpStream as TcpStreamAsync;
use tokio::select;
use tokio_util::sync::CancellationToken;
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

pub fn create_target(hostname: String, proto: String, start_port: u16, end_port: u16) -> Target {
    Target {
        hostname: hostname,
        proto: proto,
        start_port: start_port,
        end_port: end_port,
    }
}

/// Asynchronous TCP connect returning port and state or err
pub async fn inspect_port_async(
    hostname: String,
    port: u16,
    token: CancellationToken,
) -> eyre::Result<(u16, bool), io::Error> {
    let ip = hostname.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port);
    select! {
        port_result = TcpStreamAsync::connect(addr) => {
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
                    debug!("Scanning TCP port: {} returned error: {}.", port, e);
                    token.cancel();
                    Err(e)
                }
            }
        }
        _ = token.cancelled() => {
            debug!("token cancalled for port {}", port);
            Err(io::Error::new(io::ErrorKind::Interrupted, "portscan interrupted"))
        }
    }
}

pub async fn scan_target(
    target: Target,
    port_map: &mut HashMap<u16, bool>,
) -> eyre::Result<&HashMap<u16, bool>, io::Error> {
    let token = CancellationToken::new();
    let mut futures = Vec::with_capacity(target.end_port.into());

    info!(
        "Scanning target {} over {} on ports {}-{}",
        target.hostname, target.proto, target.start_port, target.end_port
    );
    for port in target.start_port..target.end_port {
        let cloned_token = token.clone();
        let hostname = target.hostname.to_string();
        let future =
            tokio::task::spawn(
                async move { inspect_port_async(hostname, port, cloned_token).await },
            );
        futures.push(future);
    }

    for result in future::join_all(futures).await {
        match result {
            Ok(Ok((port, open))) => {
                port_map.entry(port).or_insert(open);
            }
            Ok(Err(e)) => {
                info!("Are you sure the target is alive?");
                token.cancel();
                return Err(e);
            }
            Err(_) => {
                token.cancel();
                return Err(io::Error::new(io::ErrorKind::Other, "Task join error"));
            }
        }
    }
    Ok(port_map)
}
