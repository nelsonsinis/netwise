extern crate pnet;
mod utils;

use std::env::args;

use pnet::datalink::NetworkInterface;
use utils::{
    constants::errors::Messages,
    network::{get_interface::get_interface, scan_network::scan_network},
};

fn get_interface_from_args(options: &[String]) -> Result<NetworkInterface, &'static str> {
    let iface_index = options
        .iter()
        .position(|item| item == "-i" || item == "--iface");

    match iface_index {
        Some(value) => {
            if let Some(iface_value) = options.get(value + 1) {
                Ok(get_interface(iface_value)?)
            } else {
                Err(Messages::InvalidInterface.as_str())
            }
        }
        None => Err(Messages::NoIfaceParameterProvided.as_str()),
    }
}

fn main() {
    let options: Vec<String> = args().collect();

    let interface: NetworkInterface = match get_interface_from_args(&options) {
        Ok(value) => value,
        Err(error) => panic!("{}", error),
    };

    println!("Interface: {:?}", interface);
    scan_network(&interface);
}
