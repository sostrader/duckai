use cidr::IpCidr;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::net::IpAddr;
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Proxy {
    /// Upstream proxy, supports http, https, socks4, socks5, socks5h
    URL(Url),
    /// Bind to interface, supports ipv4, ipv6
    Interface(IpAddr),
    /// Bind to ipv6/ipv4 CIDR, ramdomly generate ipv4/ipv6 address
    CIDR(IpCidr),
}

impl From<Url> for Proxy {
    fn from(url: Url) -> Self {
        Proxy::URL(url)
    }
}

impl From<IpAddr> for Proxy {
    fn from(ip_addr: IpAddr) -> Self {
        Proxy::Interface(ip_addr)
    }
}

impl From<IpCidr> for Proxy {
    fn from(cidr: IpCidr) -> Self {
        Proxy::CIDR(cidr)
    }
}
