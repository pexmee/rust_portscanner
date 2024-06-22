use std::net::TcpStream;
use std::io::prelude::*;
use std::str;

fn main() -> std::io::Result<()>{ 

    let mut stream: TcpStream = match TcpStream::connect("217.160.94.169:21"){
        Ok(v) => v,
        Err(e) => panic!("Could not connect {e}"),
    };
    let mut buf:[u8; 1024] = [0;1024];

    let res: usize = match stream.peek(&mut buf){
        Ok(v) => v,
        Err(e) => panic!("Could not peek {e}"),
    };
    println!("peek length: {res}");

    let mut buf = [0;1024];
    let len: usize = match stream.read(&mut buf){
        Ok(v) => v,
        Err(e) => panic!("Could not read {e}"),
    };
    println!("length: {len}");

    let s: &str = match str::from_utf8(&buf){
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 Sequence {e}"),
    };

    println!("{}", s);
  
    Ok(())
}