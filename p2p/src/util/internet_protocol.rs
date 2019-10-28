use std::{net, str};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InternetProtocol {
    Any,
    IpV4,
    IpV6,
}

impl Default for InternetProtocol {
    fn default() -> Self {
        InternetProtocol::Any
    }
}

impl str::FromStr for InternetProtocol {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ipv4" => Ok(InternetProtocol::IpV4),
            "ipv6" => Ok(InternetProtocol::IpV6),
            _ => Err("Invalid internet protocol"),
        }
    }
}

impl InternetProtocol {
    pub fn is_allowed(&self, addr: &net::SocketAddr) -> bool {
        match *self {
            InternetProtocol::Any => true,
            InternetProtocol::IpV4 => match *addr {
                net::SocketAddr::V4(_) => true,
                _ => false,
            },
            InternetProtocol::IpV6 => match *addr {
                net::SocketAddr::V6(_) => true,
                _ => false,
            },
        }
    }
}
