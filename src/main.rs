use ping::dgramsock::ping;
use spinach::Spinach;
use std::net::Ipv4Addr;
use std::sync::mpsc;
use std::thread;
use utils::network;

mod utils;

extern crate ping;

fn main() {
    let local_ip = network::local_ip::LocalIP::new();
    let (tx, rx) = mpsc::channel();

    let spinner = Spinach::new("Searching devices in the local network...");

    for last_part in 0..=255 {
        let tx = tx.clone();
        let ip_parts = local_ip.as_vec();

        thread::spawn(move || {
            match ping(
                std::net::IpAddr::V4(Ipv4Addr::new(
                    *ip_parts.get(0).unwrap(),
                    *ip_parts.get(1).unwrap(),
                    *ip_parts.get(2).unwrap(),
                    last_part,
                )),
                None,
                None,
                None,
                Some(1),
                None,
            ) {
                Ok(_) => {
                    tx.send(Some(format!(
                        "{}.{}.{}.{}",
                        *ip_parts.get(0).unwrap(),
                        *ip_parts.get(1).unwrap(),
                        *ip_parts.get(0).unwrap(),
                        last_part
                    )))
                    .unwrap();
                }
                Err(_) => tx.send(None).unwrap(),
            };
        });
    }

    let mut results = Vec::new();

    for _ in 0..=255 {
        if let Some(value) = rx.recv().unwrap() {
            results.push(value);
        }
    }

    spinner.succeed(format!("Devices found: {}", results.len()));
}
