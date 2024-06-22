// use std::net::TcpStream;
// use std::io::prelude::*;
use std::vec;

/*
This should first be implemented as a TCP portscanner, and then with support to use UDP portscan.
Main should only take the arguments from the user and pass them to the functions.
*/
fn main() {
    use terminal_menu::{button, label, menu, mut_menu, run, string};
    let proto_menu = menu(vec![
        label("----------------------"),
        label("Select transport protocol"),
        label("-----------------------"),
        button("TCP"),
        button("UDP"),
        button("TCP & UDP"),
    ]);
    run(&proto_menu);
    let mm = mut_menu(&proto_menu);
    let proto = mm.selected_item_name();
    println!("Selected: {}", proto);
    let port_menu = menu(vec![
        label("----------------------"),
        label("Enter the port range (e.g 222-1337)"),
        label("-----------------------"),
        string("Ports", "1-65535", true),
        button("Confirm"),
    ]);
    run(&port_menu);
    if proto == "UDP" || proto == "TCP & UDP" {
        println!("Does not support UDP yet.")
    }
    let mm = mut_menu(&port_menu);
    println!("Ports: {}", mm.selection_value("Ports"));

    // for i in 1..7 as u32{

    // }

    // let mut stream: TcpStream = match TcpStream::connect("217.160.94.169:21"){
    //     Ok(v) => v,
    //     Err(e) => panic!("Could not connect {e}"),
    // };
    // let mut buf:[u8; 1024] = [0;1024];

    // let res: usize = match stream.peek(&mut buf){
    //     Ok(v) => v,
    //     Err(e) => panic!("Could not peek {e}"),
    // };
    // println!("peek length: {res}");

    // let mut buf = [0;1024];
    // let len: usize = match stream.read(&mut buf){
    //     Ok(v) => v,
    //     Err(e) => panic!("Could not read {e}"),
    // };
    // println!("length: {len}");

    // let s: &str = match str::from_utf8(&buf){
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 Sequence {e}"),
    // };

    // println!("{}", s);

    // Ok(())
}
