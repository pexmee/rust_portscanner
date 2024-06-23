use std::vec;
// use eyre;
use inquire::{validator::Validation, Select, Text};
use regex::Regex;
/*
This should first be implemented as a TCP portscanner, and then with support to use UDP portscan.
Main should only take the arguments from the user and pass them to the functions.
inquire
*/
mod networking;
fn main() {
    let validator = |input: &str| {
        let ip_pattern = Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$").unwrap();
        if input.is_empty() {
            Ok(Validation::Invalid("You need to enter an IP address".into()))
        } else if input == "localhost" {
            Ok(Validation::Valid)
        } else if ip_pattern.is_match(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Invalid host".into()))
        }
    };
    let hostname = Text::new("Enter the hostname: (e.g 127.0.0.1, localhost, 123.123.123.123)")
        .with_validator(validator)
        .prompt()
        .unwrap();

    let options = vec!["TCP", "UDP", "TCP & UDP"];
    let ans = Select::new("Select transport protocol", options)
        .prompt()
        .unwrap();

    let validator = |hostname: &str| {
        let port_range_pattern = Regex::new(r"\d+[-]\d+").unwrap();
        if hostname.chars().count() > 11 {
            Ok(Validation::Invalid(
                "You're only allowed 11 characters.".into(),
            ))
        } else if port_range_pattern.is_match(hostname) {
            Ok(Validation::Valid)
        } else if hostname.is_empty() {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Invalid port range.".into()))
        }
    };
    let port_range = Text::new("Enter the port range (e.g 123-1337, default 1-65535)")
        .with_validator(validator)
        .prompt()
        .unwrap();

    if port_range.is_empty() {
        let _port_range = "1-65535";
    }
    if ans == "TCP" {
        let result = networking::tcp_client::tcp_client(&hostname, 21);
        match result {
            Ok(v) => v,
            Err(e) => panic!("Error from tcp client {e}")
        }
    }

    // // for i in 1..7 as u32{

    // // }

    // println!("{}", s);
    // eyre::bail!("bajs");
    // Ok(());
}
