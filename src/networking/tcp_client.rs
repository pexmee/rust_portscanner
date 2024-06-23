use std::io::{prelude::*};
use std::net::TcpStream;
use eyre;
use std::str;

pub fn tcp_client(host: &str, port: u32) -> eyre::Result<()>{
    let mut stream = TcpStream::connect(format!("217.160.94.169:{port}"))?;
    let mut buf = [0;1024];

    let res = stream.peek(&mut buf)?;
    println!("peek length: {res}");

    let mut buf = [0;1024];
    let len = stream.read(&mut buf)?;
    println!("length: {len}");

    let s: &str = str::from_utf8(&buf)?;
    println!("banner: {s}");
    Ok(())
}