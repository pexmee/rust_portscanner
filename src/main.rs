use std::collections::HashMap;
use std::vec;
// use eyre;

use inquire::{validator::Validation, Select, Text};
use log::{error, info, warn};
use scanning::portscan::{create_target, scan_target};
use scanning::utils::port_parser;
use regex::Regex;
mod scanning;
#[tokio::main]
pub async fn main() {
    env_logger::init();

    let validator = |input: &str| {
        let ip_pattern = Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$").unwrap();
        if input.is_empty() || input == "localhost" {
            Ok(Validation::Invalid(
                "You need to enter an IP address".into(),
            ))
        } else if ip_pattern.is_match(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Invalid host".into()))
        }
    };
    let hostname = Text::new("Enter the hostname: (e.g 127.0.0.1, 123.123.123.123)")
        .with_validator(validator)
        .prompt()
        .unwrap();

    let options = vec!["TCP", "UDP", "TCP & UDP"];
    let proto = Select::new("Select transport protocol", options)
        .prompt()
        .unwrap();

    let validator = |port_range_str: &str| {
        let port_range_pattern = Regex::new(r"\d+[-]\d+").unwrap();
        if port_range_str.chars().count() > 11 {
            Ok(Validation::Invalid(
                "You're only allowed 11 characters.".into(),
            ))
        } else if port_range_pattern.is_match(port_range_str) {
            Ok(Validation::Valid)
        } else if port_range_str.is_empty() {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Invalid port range.".into()))
        }
    };
    let port_range_str = Text::new("Enter the port range (e.g 123-1337, default 1-65535)")
        .with_validator(validator)
        .prompt()
        .unwrap();
    let (start_port, end_port) = match port_parser(port_range_str) {
        Ok(ports) => ports,
        Err(e) => {
            warn!("Error while parsing port {:?}", e);
            return;
        }
    };
    info!("Scanning target {} over {} on ports {}-{}", hostname, proto, start_port, end_port);
    let target = create_target(hostname, proto.into(), start_port, end_port);
    let mut port_map = HashMap::<u16, bool>::with_capacity(end_port.into());
    match scan_target(target, &mut port_map).await{
        Ok(_) =>{
            info!("scan finished successfully")
        },
        Err(e) =>{
            info!("Scan returned with error: {}.", e)
        }
    };

}
