use ::tokio;
use eyre;
use log::{debug, info};
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::error::Error;
use std::time::Duration;
use tokio::net::TcpStream as TcpStreamAsync;
use tokio::select;
use std::collections::HashSet;
use tokio_util::sync::CancellationToken;
use lazy_static::lazy_static;
const ALLOWED_CON_ERRORS: &[io::ErrorKind] = &[
    io::ErrorKind::ConnectionRefused,
    io::ErrorKind::ConnectionAborted,
    io::ErrorKind::ConnectionReset,
];

// TOP 100 common TCP ports
lazy_static! {
    static ref COMMON_TCP_PORTS: HashSet<u16> = HashSet::<u16>::from([
        7, 9, 13, 21, 22, 23, 25, 26, 37, 53, 79, 80, 81, 88, 106, 110, 111, 113, 119, 135, 139, 143,
        144, 179, 199, 389, 427, 443, 444, 445, 465, 513, 514, 515, 543, 544, 548, 554, 587, 631, 646,
        873, 990, 993, 995, 1025, 1026, 1027, 1028, 1029, 1110, 1433, 1720, 1723, 1755, 1900, 2000,
        2001, 2049, 2121, 2717, 3000, 3128, 3306, 3389, 3986, 4899, 5000, 5009, 5051, 5060, 5101, 5190,
        5357, 5432, 5631, 5666, 5800, 5900, 6000, 6001, 6646, 7070, 8000, 8008, 8009, 8080, 8081, 8443,
        8888, 9100, 9999, 10000, 32768, 49152, 49153, 49154, 49155, 49156, 49157,
    ]);
} 
#[derive(Clone)]
pub struct Target {
    hostname: String,
    proto: String, // Not used yet because we don't have support for it
    start_port: u16,
    end_port: u16,
}

pub fn create_target(hostname: String, proto: String, start_port: u16, end_port: u16) -> Target {
    Target {
        hostname,
        proto,
        start_port,
        end_port,
    }
}
pub async fn scan_common_ports(target: &Target, ports_to_scan: &HashSet<u16>, duration: &Duration) -> Result<HashSet<u16>, Box<dyn Error>>{
    // We do not want to scan anything that isn't found in the user provided range
    let common: HashSet<u16> = ports_to_scan & &COMMON_TCP_PORTS;
    let closed_ports = match scan_target(target.clone(), &common, *duration).await {
        Ok(p) => p,
        Err(e) => {
            info!("Common portscan returned with error: {}", e);
            return Err(e.into());
        }
    };
    // Remove the ports we did find
    Ok(ports_to_scan & &(&common - &closed_ports))
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
    ports_to_scan: &HashSet<u16>,
    duration: Duration,
) -> eyre::Result<HashSet<u16>, io::Error> {
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
    let mut closed_ports = HashSet::with_capacity(ports_to_scan.len());

    // We do not want to use join_all here because then we risk blasting the target too hard.
    // Let this step occur sequentially to avoid that, and to sleep in-between.
    for future in futures {
        match future.await {
            Ok(Ok((port, open))) => {
                if !open {
                    closed_ports.insert(port);
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
