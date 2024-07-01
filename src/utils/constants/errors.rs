pub enum Messages {
    CannotPossibleToGetIPAddress,
    CannotPossibleToGetMacAdress,
}

impl Messages {
    pub fn as_str(&self) -> &'static str {
        match self {
            Messages::CannotPossibleToGetIPAddress => "Cannot possible to get IP address!",
            Messages::CannotPossibleToGetMacAdress => "Cannot possible to get MAC Address!",
        }
    }
}
