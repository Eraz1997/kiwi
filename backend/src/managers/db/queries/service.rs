use crate::error::Error;
use crate::managers::container::models::ContainerConfiguration;
use crate::managers::db::DbManager;
use crate::managers::db::models::ServiceData;

impl DbManager {
    pub async fn get_services_data(&self) -> Result<Vec<ServiceData>, Error> {
        let client = self.connection_pool.get().await?;
        let statement = client.prepare_cached("SELECT * FROM services").await?;
        let services: Result<Vec<ServiceData>, Error> = client
            .query(&statement, &[])
            .await?
            .into_iter()
            .map(ServiceData::try_from)
            .collect();
        services
    }

    pub async fn get_service_data(&self, name: &String) -> Result<Option<ServiceData>, Error> {
        let client = self.connection_pool.get().await?;
        let statement = client
            .prepare_cached("SELECT * FROM services WHERE name = $1")
            .await?;
        let service = client
            .query_opt(&statement, &[name])
            .await?
            .map(ServiceData::try_from)
            .and_then(Result::ok);
        Ok(service)
    }

    pub async fn create_service(
        &self,
        configuration: &ContainerConfiguration,
        postgres_username: &String,
        postgres_password: &String,
        redis_username: &String,
        redis_password: &String,
    ) -> Result<ServiceData, Error> {
        let exposed_port: Vec<i32> = vec![
            configuration.exposed_port.internal as i32,
            configuration.exposed_port.external as i32,
        ];
        let environment_variables: Vec<Vec<String>> = configuration
            .environment_variables
            .iter()
            .map(|variable| vec![variable.name.clone(), variable.value.clone()])
            .collect();
        let secrets: Vec<Vec<String>> = configuration
            .secrets
            .iter()
            .map(|variable| vec![variable.name.clone(), variable.value.clone()])
            .collect();

        let mut client = self.connection_pool.get().await?;
        let transaction = client.transaction().await?;

        let statement = transaction
            .prepare_cached(
                "INSERT INTO services (
                name,
                image_name,
                image_sha,
                exposed_port,
                environment_variables,
                secrets,
                stateful_volume_paths,
                postgres_username,
                postgres_password,
                redis_username,
                redis_password
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            ) RETURNING (
                name,
                image_name,
                image_sha,
                exposed_port,
                environment_variables,
                secrets,
                stateful_volume_paths,
                postgres_username,
                postgres_password,
                redis_username,
                redis_password,
                created_at,
                last_modified_at,
                last_deployed_at
            )",
            )
            .await?;
        let service_row = transaction
            .query_one(
                &statement,
                &[
                    &configuration.name,
                    &configuration.image_name,
                    &configuration.image_sha.get_value(),
                    &exposed_port,
                    &environment_variables,
                    &secrets,
                    &configuration.stateful_volume_paths,
                    &postgres_username,
                    &postgres_password,
                    &redis_username,
                    &redis_password,
                ],
            )
            .await?;
        let service = ServiceData::try_from(service_row)?;

        let statement = transaction
            .prepare_cached("CREATE USER $1 NOCREATEDB NOCREATEUSER PASSWORD $2 ENCRYPTED")
            .await?;
        transaction
            .execute(&statement, &[postgres_username, postgres_password])
            .await?;

        let statement = transaction
            .prepare_cached("CREATE DATABASE $1 WITH OWNER $1")
            .await?;
        transaction
            .execute(&statement, &[postgres_username, postgres_password])
            .await?;

        transaction.commit().await?;

        Ok(service)
    }

    pub async fn delete_service(
        &self,
        name: &String,
        postgres_username: &String,
    ) -> Result<(), Error> {
        let mut client = self.connection_pool.get().await?;
        let transaction = client.transaction().await?;

        let statement = transaction
            .prepare_cached("DELETE FROM services WHERE name = $1")
            .await?;
        transaction.execute(&statement, &[name]).await?;

        let statement = transaction.prepare_cached("DROP USER $1").await?;
        transaction
            .execute(&statement, &[postgres_username])
            .await?;

        let statement = transaction.prepare_cached("DROP DATABASE $1 FORCE").await?;
        transaction
            .execute(&statement, &[postgres_username])
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn update_service(
        &self,
        old_service: &ServiceData,
        new_configuration: &ContainerConfiguration,
    ) -> Result<ServiceData, Error> {
        let exposed_port: Vec<i32> = vec![
            new_configuration.exposed_port.internal as i32,
            new_configuration.exposed_port.external as i32,
        ];
        let environment_variables: Vec<Vec<String>> = new_configuration
            .environment_variables
            .iter()
            .map(|variable| vec![variable.name.clone(), variable.value.clone()])
            .collect();
        let secrets: Vec<Vec<String>> = new_configuration
            .secrets
            .iter()
            .map(|variable| vec![variable.name.clone(), variable.value.clone()])
            .collect();

        let client = self.connection_pool.get().await?;

        let statement = client
            .prepare_cached(
                "UPDATE services SET
                    name = $1,
                    image_name = $2,
                    image_sha = $3,
                    exposed_port = $4,
                    environment_variables = $5,
                    secrets = $6,
                    stateful_volume_paths = $7
                    last_modified_at = now(),
                    last_deployed_at = now()
                RETURNING (
                    name,
                    image_name,
                    image_sha,
                    exposed_port,
                    environment_variables,
                    secrets,
                    stateful_volume_paths,
                    postgres_username,
                    postgres_password,
                    redis_username,
                    redis_password,
                    created_at,
                    last_modified_at,
                    last_deployed_at
                ) WHERE name = $8",
            )
            .await?;
        let service_row = client
            .query_one(
                &statement,
                &[
                    &new_configuration.name,
                    &new_configuration.image_name,
                    &new_configuration.image_sha.get_value(),
                    &exposed_port,
                    &environment_variables,
                    &secrets,
                    &new_configuration.stateful_volume_paths,
                    &old_service.container_configuration.name,
                ],
            )
            .await?;
        let service = ServiceData::try_from(service_row)?;

        Ok(service)
    }

    pub async fn get_service_port(&self, name: &String) -> Result<Option<i32>, Error> {
        let client = self.connection_pool.get().await?;

        let statement = client
            .prepare_cached("SELECT exposed_port FROM services WHERE name = $1")
            .await?;
        let row = client.query_opt(&statement, &[name]).await?;

        if let Some(row) = row {
            let exposed_port = row.try_get::<&str, Vec<i32>>("exposed_port")?;
            Ok(exposed_port.get(1).cloned())
        } else {
            Ok(None)
        }
    }
}
