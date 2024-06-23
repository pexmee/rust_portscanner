// use std::io::{prelude::*};
use std::net::TcpStream;
use eyre;
use std::str;
pub fn tcp_connect(host: &str, port: u16) -> eyre::Result<(), eyre::ErrReport>{
    TcpStream::connect(format!("{host}:{port}"))?;
    Ok(())
}

