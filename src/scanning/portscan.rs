use ::tokio;
use eyre;
use log::{debug, info};
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream as TcpStreamAsync;
use tokio::select;
use tokio_util::sync::CancellationToken;
const ALLOWED_CON_ERRORS: &[io::ErrorKind] = &[
    io::ErrorKind::ConnectionRefused,
    io::ErrorKind::ConnectionAborted,
    io::ErrorKind::ConnectionReset,
];
#[derive(Clone)]
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
//TODO: Make a producer for the ports to scan that also sets PPS, and then the consumers read from that channel
pub async fn scan_target(
    target: Target,
    ports_to_scan: &Vec<u16>,
    duration: Duration,
) -> eyre::Result<Vec<u16>, io::Error> {
    let token = CancellationToken::new();
    let mut futures = Vec::with_capacity(target.end_port.into());

    info!(
        "Scanning target {} over {} on ports {}-{}",
        target.hostname, target.proto, target.start_port, target.end_port
    );
    // Only iterate over the closed ports, and map to the key
    for &port in ports_to_scan.iter() {
        let cloned_token = token.clone();
        let hostname = target.hostname.to_string();
        let future =
            tokio::task::spawn(
                async move { inspect_port_async(hostname, port, cloned_token).await },
            );
        futures.push(future);
    }
    let mut closed_ports = Vec::with_capacity(ports_to_scan.len());

    // We do not want to use join_all here because then we risk blasting the target too hard.
    // Let this step occur sequentially to avoid that, and to sleep in-between.
    for future in futures {
        match future.await {
            Ok(Ok((port, open))) => {
                if !open {
                    closed_ports.push(port);
                }
            }
            Ok(Err(e)) => {
                info!("Are you sure the target is alive?");
                token.cancel();
                return Err(e);
            }
            Err(_) => {
                debug!("Task join error, cancelling token");
                token.cancel();
                return Err(io::Error::new(io::ErrorKind::Other, "Task join error"));
            }
        }
        tokio::time::sleep(duration).await;
    }
    Ok(closed_ports)
}
