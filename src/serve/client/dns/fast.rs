use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    time::Instant,
};

use futures_util::future::join_all;
use hickory_resolver::{
    config::{LookupIpStrategy, NameServerConfigGroup, ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
use tokio::sync::OnceCell;

#[allow(clippy::declare_interior_mutable_const)]
pub const FASTEST_DNS_CONFIG: OnceCell<ResolverConfig> = OnceCell::const_new();

/// IP addresses for Tencent Public DNS
pub const TENCENT_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(119, 29, 29, 29)),
    IpAddr::V4(Ipv4Addr::new(119, 29, 29, 30)),
    IpAddr::V6(Ipv6Addr::new(0x2402, 0x4e00, 0, 0, 0, 0, 0, 0x1)),
];

/// IP addresses for Aliyun Public DNS
pub const ALIYUN_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(223, 5, 5, 5)),
    IpAddr::V4(Ipv4Addr::new(223, 6, 6, 6)),
    IpAddr::V6(Ipv6Addr::new(0x2400, 0x3200, 0, 0, 0, 0, 0, 0x1)),
];

pub trait ResolverConfigExt {
    fn tencent() -> ResolverConfig;
    fn aliyun() -> ResolverConfig;
}

impl ResolverConfigExt for ResolverConfig {
    fn tencent() -> ResolverConfig {
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(TENCENT_IPS, 53, true),
        )
    }

    fn aliyun() -> ResolverConfig {
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(ALIYUN_IPS, 53, true),
        )
    }
}

/// Fastest DNS resolver
pub async fn load_fastest_dns() -> crate::Result<ResolverConfig> {
    let mut tasks = Vec::new();

    let mut opts = ResolverOpts::default();
    opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;

    let configs = vec![
        ResolverConfig::google(),
        ResolverConfig::quad9(),
        ResolverConfig::cloudflare(),
        ResolverConfig::tencent(),
        ResolverConfig::aliyun(),
    ];

    for config in configs {
        let resolver = TokioAsyncResolver::tokio(config.clone(), opts.clone());
        let task = async move {
            let start = Instant::now();
            let ips = resolver.lookup_ip("duckduckgo.com").await?;
            let elapsed = start.elapsed();
            let ips = ips.iter().collect::<Vec<_>>();
            tracing::debug!("Fastest DNS resovler: {ips:?} ({elapsed:?})");
            Ok((elapsed, config))
        };
        tasks.push(task);
    }

    // Join all tasks and return the fastest DNS
    let r = join_all(tasks)
        .await
        .into_iter()
        .collect::<crate::Result<Vec<_>>>()?;

    if let Some((elapsed, conf)) = r.into_iter().min_by_key(|(elapsed, _)| *elapsed) {
        // '\n*' split fastest_dns_group
        let mut fastest_dns_group = conf
            .name_servers()
            .iter()
            .map(|ns| ns.socket_addr.to_string())
            .collect::<Vec<_>>();

        // this removes all duplicates
        fastest_dns_group.dedup();

        tracing::info!(
            "Fastest DNS group ({elapsed:?}):\n* {}",
            fastest_dns_group.join("\n* ")
        );

        return Ok(conf);
    }

    Ok(ResolverConfig::default())
}
