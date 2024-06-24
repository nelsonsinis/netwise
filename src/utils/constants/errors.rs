pub enum Messages {
    InvalidInterface,
    NoIfaceParameterProvided,
}

impl Messages {
    pub fn as_str(&self) -> &'static str {
        match self {
            Messages::InvalidInterface => "Please, provide a valid interface!",
            Messages::NoIfaceParameterProvided => "Please, provide -i or -iface parameter!",
        }
    }
}
