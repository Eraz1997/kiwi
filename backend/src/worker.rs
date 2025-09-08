use std::sync::Arc;

use tokio::{
    select,
    sync::Mutex,
    time::{Duration, sleep},
};

use crate::{
    managers::{dynamic_dns::DynamicDnsManager, lets_encrypt::LetsEncryptManager},
    models::ServerAction,
};

pub struct Worker {
    dynamic_dns_manager: Arc<Mutex<Option<DynamicDnsManager>>>,
    lets_encrypt_manager: Arc<Mutex<LetsEncryptManager>>,
}

impl Worker {
    pub fn new(
        dynamic_dns_manager: Arc<Mutex<Option<DynamicDnsManager>>>,
        lets_encrypt_manager: Arc<Mutex<LetsEncryptManager>>,
    ) -> Self {
        tracing::info!("side worker initialised");

        Self {
            dynamic_dns_manager,
            lets_encrypt_manager,
        }
    }

    pub async fn start(&self) -> ServerAction {
        select! {
            _ = self.refresh_dns() => ServerAction::CloseDueToUnexpectedError,
            worker_return_action = self.refresh_tls_certificates() => worker_return_action,
        }
    }

    async fn refresh_dns(&self) {
        loop {
            sleep(Duration::from_secs(60)).await;
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

    async fn refresh_tls_certificates(&self) -> ServerAction {
        loop {
            sleep(Duration::from_secs(60)).await;
            match self
                .lets_encrypt_manager
                .lock()
                .await
                .was_certificate_updated()
                .await
            {
                Ok(true) => {
                    tracing::info!("tls certificates have changed, server restart trigger issued");
                    return ServerAction::RestartWithoutDependenciesInit;
                }
                Ok(false) => {}
                Err(error) => {
                    tracing::error!("error checking any tls certificate updates: {}", error);
                }
            }
        }
    }
}
