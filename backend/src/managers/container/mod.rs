use std::collections::HashMap;

use crate::error::Error;
#[allow(deprecated)]
use bollard::volume::CreateVolumeOptions;
use bollard::{
    Docker,
    query_parameters::{
        CreateContainerOptionsBuilder, CreateImageOptionsBuilder, ListContainersOptionsBuilder,
        RemoveContainerOptionsBuilder, StartContainerOptions, StopContainerOptions,
    },
    secret::{ContainerCreateBody, ContainerSummaryStateEnum, HostConfig, PortBinding},
};
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

        Ok(Self { client })
    }

    pub async fn start_container(
        &self,
        configuration: &ContainerConfiguration,
    ) -> Result<(), Error> {
        let volumes: Vec<(String, String)> = configuration
            .stateful_volume_paths
            .iter()
            .map(|path| {
                (
                    configuration.clone().get_stateful_volume_id(path),
                    path.clone(),
                )
            })
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
            .from_image(image_tag.as_str())
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
            .map(|env_var| format!("{}={}", env_var.name, env_var.value))
            .collect();
        let port_bindings: HashMap<String, Option<Vec<PortBinding>>> = configuration
            .exposed_ports
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
        let exposed_ports: HashMap<String, HashMap<(), ()>> = configuration
            .exposed_ports
            .iter()
            .map(|port| (port.internal.to_string(), HashMap::new()))
            .collect();
        let volume_bindings = configuration
            .stateful_volume_paths
            .iter()
            .map(|path| {
                format!(
                    "{}:{}",
                    configuration.clone().get_stateful_volume_id(path),
                    path
                )
            })
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
}
