// use std::io::{prelude::*};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::TcpStream;
use std::time::Duration;
use eyre;
use std::str;
pub fn tcp_connect(host: &str, port: u16) -> eyre::Result<(), eyre::ErrReport>{
    let ip = host.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port);
    // TcpStream::connect(format!("{host}:{port}"))?;
    TcpStream::connect_timeout(&addr, Duration::new(0, 1_000_000_000))?;
    Ok(())
}

