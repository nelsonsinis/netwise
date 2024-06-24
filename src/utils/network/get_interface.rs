use pnet::datalink::{self, NetworkInterface};

use crate::utils::constants::errors::Messages;

pub fn get_interface(name: &str) -> Result<NetworkInterface, &'static str> {
    match datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == name)
    {
        Some(value) => Ok(value),
        None => Err(Messages::InvalidInterface.as_str()),
    }
}
