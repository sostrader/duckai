use cidr::IpCidr;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::net::IpAddr;
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Proxies {
    /// Upstream proxy, supports http, https, socks4, socks5, socks5h
    Proxy(Url),
    /// Bind to interface, supports ipv4, ipv6
    Interface(IpAddr),
    /// Bind to ipv6/ipv4 CIDR, ramdomly generate ipv4/ipv6 address
    CIDR(IpCidr),
}

impl From<Url> for Proxies {
    fn from(url: Url) -> Self {
        Proxies::Proxy(url)
    }
}

impl From<IpAddr> for Proxies {
    fn from(ip_addr: IpAddr) -> Self {
        Proxies::Interface(ip_addr)
    }
}

impl From<IpCidr> for Proxies {
    fn from(cidr: IpCidr) -> Self {
        Proxies::CIDR(cidr)
    }
}
