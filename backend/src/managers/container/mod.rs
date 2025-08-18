use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

use crate::error::Error;
use crate::managers::container::models::Log;
use bollard::container::LogOutput;
use bollard::query_parameters::LogsOptionsBuilder;
#[allow(deprecated)]
use bollard::volume::CreateVolumeOptions;
#[allow(deprecated)]
use bollard::volume::RemoveVolumeOptions;
#[allow(deprecated)]
use bollard::{
    Docker,
    network::{ConnectNetworkOptions, CreateNetworkOptions, DisconnectNetworkOptions},
    query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, ListContainersOptionsBuilder,
        ListNetworksOptions, RemoveContainerOptionsBuilder, StartContainerOptions,
        StopContainerOptions,
    },
    secret::{ContainerCreateBody, ContainerSummaryStateEnum, HostConfig, Network, PortBinding},
};
use chrono::NaiveDateTime;
use futures::TryStreamExt;
use futures::stream::StreamExt;
use models::ContainerConfiguration;

pub mod error;
pub mod models;

#[derive(Clone)]
pub struct ContainerManager {
    client: Docker,
}

impl ContainerManager {
    pub async fn new() -> Result<Self, Error> {
        let client = Docker::connect_with_local_defaults()?;

        let _connection_test = client.version().await?;

        tracing::info!("docker client initialised");

        let list_options = ListContainersOptionsBuilder::new().all(true).build();
        let running_containers = client.list_containers(Some(list_options)).await?;

        if running_containers.is_empty() {
            tracing::info!("no running containers found, skipping reset");
        } else {
            for container in running_containers.iter() {
                let container_id = container
                    .id
                    .clone()
                    .ok_or(Error::container_id_not_found())?;
                match container.state {
                    Some(ContainerSummaryStateEnum::CREATED)
                    | Some(ContainerSummaryStateEnum::RUNNING)
                    | Some(ContainerSummaryStateEnum::RESTARTING) => {
                        client
                            .stop_container(container_id.as_str(), None::<StopContainerOptions>)
                            .await?;
                    }
                    _ => {}
                }
                let remove_options = RemoveContainerOptionsBuilder::new().force(true).build();
                client
                    .remove_container(container_id.as_str(), Some(remove_options))
                    .await?;
            }
            tracing::info!(
                "reset docker status: stopped and removed {} containers",
                running_containers.len()
            );
        }
        let networks: Vec<Network> = client
            .list_networks(None::<ListNetworksOptions>)
            .await?
            .into_iter()
            .filter(|network| {
                network
                    .name
                    .clone()
                    .filter(|name| name == "bridge" || name == "none" || name == "host")
                    .is_none()
            })
            .collect();

        if networks.is_empty() {
            tracing::info!("no networks found, skipping reset");
        } else {
            for network in networks.iter() {
                client
                    .remove_network(
                        network
                            .name
                            .clone()
                            .ok_or(Error::network_name_not_found())?
                            .as_str(),
                    )
                    .await?;
            }
            tracing::info!("reset docker status: removed {} networks", networks.len());
        }

        Ok(Self { client })
    }

    pub async fn start_container(
        &self,
        configuration: &ContainerConfiguration,
    ) -> Result<(), Error> {
        let volumes: Vec<(String, String)> = configuration
            .stateful_volume_paths
            .iter()
            .map(|path| (configuration.get_stateful_volume_id(path), path.clone()))
            .collect();

        for (volume_id, _) in volumes.iter() {
            let volume_details = self.client.inspect_volume(volume_id).await;
            let needs_creation = match volume_details {
                Err(bollard::errors::Error::DockerResponseServerError {
                    status_code: 404,
                    message: _,
                }) => Ok(true),
                Err(error) => Err(error),
                Ok(_) => Ok(false),
            }?;

            if needs_creation {
                #[allow(deprecated)]
                let options = CreateVolumeOptions {
                    name: volume_id.clone(),
                    ..Default::default()
                };
                self.client.create_volume(options).await?;
                tracing::info!("volume {} created", volume_id);
            } else {
                tracing::info!("volume {} already exists", volume_id);
            }
        }

        let image_tag = format!(
            "{}@sha256:{}",
            configuration.image_name,
            configuration.image_sha.get_value()
        );

        let create_image_options = CreateImageOptionsBuilder::new()
            .from_image(&image_tag)
            .build();
        let mut image_pull_stream =
            self.client
                .create_image(Some(create_image_options), None, None);

        tracing::info!("started pulling image {}", image_tag);

        while let Some(pull_result) = image_pull_stream.next().await {
            pull_result?;
        }

        tracing::info!("pulled image {}", image_tag);

        let options = CreateContainerOptionsBuilder::new()
            .name(&configuration.name)
            .build();

        let env_vars: Vec<String> = configuration
            .environment_variables
            .iter()
            .chain(&configuration.secrets)
            .chain(&configuration.internal_secrets)
            .map(|env_var| format!("{}={}", env_var.name, env_var.value))
            .collect();
        let port_bindings: HashMap<String, Option<Vec<PortBinding>>> =
            [configuration.exposed_port.clone()]
                .iter()
                .map(|port| {
                    (
                        port.internal.to_string(),
                        Some(vec![PortBinding {
                            host_ip: Some("127.0.0.1".to_string()),
                            host_port: Some(port.external.to_string()),
                        }]),
                    )
                })
                .collect();
        let exposed_ports: HashMap<String, HashMap<(), ()>> = [configuration.exposed_port.clone()]
            .iter()
            .map(|port| (port.internal.to_string(), HashMap::new()))
            .collect();
        let volume_bindings = configuration
            .stateful_volume_paths
            .iter()
            .map(|path| format!("{}:{}", configuration.get_stateful_volume_id(path), path))
            .collect();

        let configuration_body = ContainerCreateBody {
            env: Some(env_vars),
            exposed_ports: Some(exposed_ports),
            host_config: Some(HostConfig {
                port_bindings: Some(port_bindings),
                binds: Some(volume_bindings),
                ..Default::default()
            }),
            image: Some(image_tag),
            ..Default::default()
        };

        self.client
            .create_container(Some(options), configuration_body)
            .await?;

        self.client
            .start_container(&configuration.name, None::<StartContainerOptions>)
            .await?;

        tracing::info!("container {} started", configuration.name);

        Ok(())
    }

    pub async fn create_and_attach_network_for_container(
        &self,
        configuration: &ContainerConfiguration,
    ) -> Result<(), Error> {
        self.detach_and_remove_any_network(&configuration.name)
            .await?;

        #[allow(deprecated)]
        let options = CreateNetworkOptions {
            name: configuration.name.clone(),
            ..Default::default()
        };
        self.client.create_network(options).await?;

        for container_name in ["kiwi-postgres", "kiwi-redis", &configuration.name] {
            #[allow(deprecated)]
            let options = ConnectNetworkOptions {
                container: container_name,
                ..Default::default()
            };
            self.client
                .connect_network(configuration.name.as_str(), options)
                .await?;
        }

        Ok(())
    }

    pub async fn get_container_status(&self, name: &str) -> Result<String, Error> {
        let status = self
            .get_container_status_enum(name)
            .await?
            .map(|status| status.to_string())
            .unwrap_or("Unknown".to_string());
        Ok(status)
    }

    pub async fn get_container_logs(
        &self,
        name: &str,
        from_date: NaiveDateTime,
        to_date: NaiveDateTime,
    ) -> Result<Vec<Log>, Error> {
        let options = LogsOptionsBuilder::new()
            .stdout(true)
            .stderr(true)
            .timestamps(true)
            .since(from_date.and_utc().timestamp() as i32)
            .until(to_date.and_utc().timestamp() as i32)
            .build();
        let logs_raw: Vec<LogOutput> = self.client.logs(name, Some(options)).try_collect().await?;
        let logs: Vec<Log> = logs_raw.into_iter().map(|log| log.into()).collect();

        Ok(logs)
    }

    pub async fn stop_and_remove_container(&self, name: &str) -> Result<(), Error> {
        self.detach_and_remove_any_network(name).await?;
        let status = self.get_container_status_enum(name).await?;

        match status {
            Some(ContainerSummaryStateEnum::CREATED)
            | Some(ContainerSummaryStateEnum::RUNNING)
            | Some(ContainerSummaryStateEnum::RESTARTING) => {
                self.client
                    .stop_container(name, None::<StopContainerOptions>)
                    .await?;
            }
            _ => {}
        }

        let options = RemoveContainerOptionsBuilder::new().force(true).build();
        self.client.remove_container(name, Some(options)).await?;

        Ok(())
    }

    pub async fn remove_volumes(
        &self,
        configuration: &ContainerConfiguration,
    ) -> Result<(), Error> {
        let volume_ids: Vec<String> = configuration
            .stateful_volume_paths
            .iter()
            .map(|path| configuration.get_stateful_volume_id(path))
            .collect();

        for volume_id in volume_ids {
            #[allow(deprecated)]
            let options = RemoveVolumeOptions { force: true };
            self.client.remove_volume(&volume_id, Some(options)).await?;
        }

        Ok(())
    }

    pub fn is_local_port_free(port: &u16) -> bool {
        let ipv4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, *port);
        TcpListener::bind(ipv4).is_ok()
    }

    async fn detach_and_remove_any_network(&self, name: &str) -> Result<(), Error> {
        let network_to_delete = self
            .client
            .list_networks(None::<ListNetworksOptions>)
            .await?
            .into_iter()
            .find(|network| {
                network
                    .name
                    .clone()
                    .filter(|network_name| *network_name == name)
                    .is_some()
            });

        if let Some(network_to_delete) = network_to_delete {
            let network_name = network_to_delete
                .name
                .clone()
                .ok_or(Error::network_name_not_found())?;
            if let Some(attached_containers) = network_to_delete.containers {
                for container in attached_containers.values() {
                    #[allow(deprecated)]
                    let options = DisconnectNetworkOptions {
                        container: container
                            .name
                            .clone()
                            .ok_or(Error::container_id_not_found())?,
                        force: true,
                    };
                    self.client
                        .disconnect_network(network_name.as_str(), options)
                        .await?;
                }
            }
            self.client.remove_network(network_name.as_str()).await?;
        }

        Ok(())
    }

    async fn get_container_status_enum(
        &self,
        name: &str,
    ) -> Result<Option<ContainerSummaryStateEnum>, Error> {
        let list_options = ListContainersOptionsBuilder::new().all(true).build();
        let containers = self.client.list_containers(Some(list_options)).await?;

        let container = containers
            .into_iter()
            .find(|container| {
                container
                    .names
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .any(|container_name| *container_name == name)
            })
            .ok_or(Error::container_id_not_found())?;

        Ok(container.state)
    }
}
