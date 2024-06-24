use std::net::Ipv4Addr;

use pnet::datalink::{self, Channel::Ethernet, NetworkInterface};

pub fn scan_network(interface: &NetworkInterface) {
    let (mut tx, mut rx) = match datalink::channel(interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhadled channel type"),
        Err(e) => panic!("Failed to create datalink channel: {}", e),
    };

    let source_mac: datalink::MacAddr = interface.mac.unwrap();
    let source_ip = match interface.ips.iter().find(|ip| ip.is_ipv4()) {
        Some(value) => value.ip(),
        None => panic!("No IP associated!"),
    };

    println!("{source_ip}");
}
