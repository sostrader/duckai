mod build;
mod dns;
mod pool;
#[cfg(target_os = "linux")]
mod route;

use crate::config;
use pool::Pool;
use std::{ops::Deref, sync::Arc};

/// Client round-robin balancer
#[derive(Clone)]
pub struct ClientLoadBalancer {
    pool: Arc<Pool>,
    _priv: (),
}

impl ClientLoadBalancer {
    pub async fn new(conf: config::Config) -> Self {
        Self {
            pool: Arc::new(Pool::new(conf).await),
            _priv: (),
        }
    }
}

impl Deref for ClientLoadBalancer {
    type Target = Arc<Pool>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}
