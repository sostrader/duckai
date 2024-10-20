//! DNS resolution via the [trust_dns_resolver](https://github.com/bluejekyll/trust-dns) crate
mod fast;

use std::{
    io,
    net::SocketAddr,
    sync::{Arc, OnceLock},
};

pub use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::{
    config::{LookupIpStrategy, NameServerConfigGroup},
    lookup_ip::LookupIpIntoIter,
    system_conf, TokioAsyncResolver,
};
use moka::future::Cache;
use rquest::dns::{Addrs, Name, Resolve, Resolving};
use tokio::sync::OnceCell;

static DNS_RESOLVER: OnceLock<Cache<u8, Arc<HickoryDnsResolver>>> = OnceLock::new();

fn from_strategy(strategy: LookupIpStrategy) -> u8 {
    match strategy {
        LookupIpStrategy::Ipv4Only => 0,
        LookupIpStrategy::Ipv6Only => 1,
        LookupIpStrategy::Ipv4AndIpv6 => 2,
        LookupIpStrategy::Ipv6thenIpv4 => 3,
        LookupIpStrategy::Ipv4thenIpv6 => 4,
    }
}

/// Create a DNS resolver
pub async fn get_dns_resolver(ip_strategy: LookupIpStrategy) -> Arc<HickoryDnsResolver> {
    // maybe DNS_RESOLVER is not initialized
    let cache = DNS_RESOLVER.get_or_init(|| Cache::builder().max_capacity(5).build());
    // init dns resolver cache
    cache
        .get_with(from_strategy(ip_strategy), async move {
            Arc::new(HickoryDnsResolver::new(ip_strategy))
        })
        .await
}

/// Wrapper around an `AsyncResolver`, which implements the `Resolve` trait.
#[derive(Debug, Clone)]
pub(crate) struct HickoryDnsResolver {
    /// Since we might not have been called in the context of a
    /// Tokio Runtime in initialization, so we must delay the actual
    /// construction of the resolver.
    state: Arc<OnceCell<TokioAsyncResolver>>,
    /// The DNS strategy to use when resolving addresses.
    ip_strategy: LookupIpStrategy,
}

impl HickoryDnsResolver {
    /// Create a new `TrustDnsResolver` with the default configuration,
    /// which reads from `/etc/resolve.conf`.
    pub(crate) fn new(ip_strategy: LookupIpStrategy) -> Self {
        Self {
            state: Arc::new(OnceCell::new()),
            ip_strategy,
        }
    }
}

struct SocketAddrs {
    iter: LookupIpIntoIter,
}

impl Resolve for HickoryDnsResolver {
    fn resolve(&self, name: Name) -> Resolving {
        let resolver = self.clone();
        Box::pin(async move {
            let resolver = resolver
                .state
                .get_or_try_init(|| new_resolver(resolver.ip_strategy))
                .await?;
            let lookup = resolver.lookup_ip(name.as_str()).await?;
            let addrs: Addrs = Box::new(SocketAddrs {
                iter: lookup.into_iter(),
            });
            Ok(addrs)
        })
    }
}

impl Iterator for SocketAddrs {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ip_addr| SocketAddr::new(ip_addr, 0))
    }
}

/// Create a new resolver with the default configuration,
/// which reads from `/etc/resolve.conf`.
async fn new_resolver(ip_strategy: LookupIpStrategy) -> io::Result<TokioAsyncResolver> {
    // If we can't read the system conf, just use the defaults.
    let (default_config, mut opts) = match system_conf::read_system_conf() {
        Ok((config, opts)) => (config, opts),
        Err(err) => {
            tracing::warn!("Error reading DNS system conf: {}", err);
            // Use Google DNS, Cloudflare DNS and Quad9 DNS
            let mut group = NameServerConfigGroup::new();

            // Google DNS
            group.extend(NameServerConfigGroup::google().into_inner());

            // Cloudflare DNS
            group.extend(NameServerConfigGroup::cloudflare().into_inner());

            // Quad9 DNS
            group.extend(NameServerConfigGroup::quad9().into_inner());

            let config = ResolverConfig::from_parts(None, vec![], group);
            (config, ResolverOpts::default())
        }
    };

    // Check /ect/hosts file before dns requery (only works for unix like OS)
    opts.use_hosts_file = true;
    // The ip_strategy for the Resolver to use when lookup Ipv4 or Ipv6 addresses
    opts.ip_strategy = ip_strategy;

    // Use built-in fastest DNS group
    let config = fast::FASTEST_DNS_CONFIG
        .get_or_try_init(fast::load_fastest_dns)
        .await
        .cloned()
        .unwrap_or(default_config);

    Ok(TokioAsyncResolver::tokio(config, opts))
}
