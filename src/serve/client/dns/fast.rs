use std::time::Instant;

use futures_util::future::join_all;
use hickory_resolver::{
    config::{LookupIpStrategy, ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
use tokio::sync::OnceCell;

pub const FASTEST_DNS_CONFIG: OnceCell<ResolverConfig> = OnceCell::const_new();

/// Fastest DNS resolver
pub async fn load_fastest_dns() -> crate::Result<ResolverConfig> {
    let mut tasks = Vec::new();

    let mut opts = ResolverOpts::default();
    opts.ip_strategy = LookupIpStrategy::Ipv4AndIpv6;

    let configs = vec![
        ResolverConfig::google(),
        ResolverConfig::quad9(),
        ResolverConfig::cloudflare(),
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
