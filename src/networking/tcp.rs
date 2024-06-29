// use std::io::{prelude::*};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::net::TcpStream;
use std::time::Duration;
use eyre::{self, bail};
use std::str;
use std::io::ErrorKind;
pub enum ConnectonState {
    Open(),
    Closed(),
}
pub fn tcp_connect(host: &str, port: u16) -> eyre::Result<ConnectonState>{
    let ip = host.parse::<Ipv4Addr>().unwrap();
    let addr = SocketAddr::new(IpAddr::V4(ip), port);
    // TcpStream::connect(format!("{host}:{port}"))?;
    let port_result = TcpStream::connect_timeout(&addr, Duration::new(0, 1_000_000_000));
    match port_result{
        Err(ref e) if [ErrorKind::PermissionDenied].contains(&e.kind()) => {
            bail!("Permission denied")
        },
        Ok(_) =>{
            Ok(ConnectonState::Open())
        },
        Err(_) => {
            Ok(ConnectonState::Closed())
        },
    }
}

// {
//     let port_result = todo!();
//     match port_result {
//         ConnectonStat::Open(port, addr) => {

//         },
//         ConnectonStat::Closed() => {

//         }
//         Err(),
//     }
// }
/*
Open a TCP connection to 127.0.0.1:8080:

use std::net::TcpStream;

if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
    println!("Connected to the server!");
} else {
    println!("Couldn't connect to server...");
}
Open a TCP connection to 127.0.0.1:8080. If the connection fails, open a TCP connection to 127.0.0.1:8081:

use std::net::{SocketAddr, TcpStream};

let addrs = [
    SocketAddr::from(([127, 0, 0, 1], 8080)),
    SocketAddr::from(([127, 0, 0, 1], 8081)),
];
if let Ok(stream) = TcpStream::connect(&addrs[..]) {
    println!("Connected to the server!");
} else {
    println!("Couldn't connect to server...");
}
*/