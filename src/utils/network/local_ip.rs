use std::net::IpAddr;

use local_ip_address::local_ip;

#[derive(Debug)]
pub struct LocalIP {
    ip: IpAddr,
}

impl LocalIP {
    pub fn new() -> Self {
        match local_ip() {
            Ok(value) => Self { ip: value },
            Err(error) => panic!("It's not possible to get local IP"),
        }
    }
    pub fn as_vec(&self) -> Vec<u8> {
        self.ip
            .to_string()
            .split(".")
            .map(|part| part.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
    }
}
