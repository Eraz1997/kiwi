use std::sync::Arc;

use tokio::{
    sync::Mutex,
    time::{Duration, sleep},
};

use crate::managers::dynamic_dns::DynamicDnsManager;

pub struct Worker {
    dynamic_dns_manager: Arc<Mutex<Option<DynamicDnsManager>>>,
}

impl Worker {
    pub fn new(dynamic_dns_manager: Arc<Mutex<Option<DynamicDnsManager>>>) -> Self {
        tracing::info!("side worker initialised");

        Self {
            dynamic_dns_manager,
        }
    }

    pub async fn start(&self) {
        loop {
            sleep(Duration::from_secs(60)).await;
            self.refresh_dns().await;
        }
    }

    async fn refresh_dns(&self) {
        if let Some(mut dynamic_dns_manager) = self.dynamic_dns_manager.lock().await.take() {
            let refresh_result = dynamic_dns_manager.refresh().await;

            if let Err(error) = refresh_result {
                tracing::error!("refresh dns job failed: {}", error);
            }
        } else {
            tracing::info!("skipping refresh dns job as dynamic dns is not configured");
        }
    }
}
