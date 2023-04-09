use std::{net::Ipv4Addr, str::FromStr};

use enet::Address;

pub(crate) trait GetSocketAddress {
    fn strip_protocol(&self) -> String;
    fn parse_integer_address(&self, addr: u32) -> Ipv4Addr;
    fn get_address(&self) -> Address;
}

impl GetSocketAddress for &str {
    fn strip_protocol(&self) -> String {
        self.trim_start_matches("aos://").to_string()
    }

    fn parse_integer_address(&self, addr: u32) -> Ipv4Addr {
        Ipv4Addr::new(
            (addr & 255) as u8,
            ((addr >> 8) & 255) as u8,
            ((addr >> 16) & 255) as u8,
            ((addr >> 24) & 255) as u8,
        )
    }

    fn get_address(&self) -> Address {
        let address = self.strip_protocol();

        let (ip_string, port_string) = match address.find(':') {
            Some(pos) => address.split_at(pos),
            None => (address.as_str(), "32887"),
        };

        let port: u16 = port_string
            .trim_start_matches(':')
            .parse()
            .expect("Invalid port number");

        let ip = if ip_string.contains('.') {
            Ipv4Addr::from_str(ip_string).expect("Invalid IP address")
        } else {
            let ip_int: u32 = ip_string.parse().expect("Invalid IP address");
            self.parse_integer_address(ip_int)
        };

        Address::new(ip, port)
    }
}
