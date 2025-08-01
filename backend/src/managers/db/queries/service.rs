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
        let exposed_ports: Vec<Vec<i32>> = configuration
            .exposed_ports
            .iter()
            .map(|port| vec![port.internal as i32, port.external as i32])
            .collect();
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
                exposed_ports,
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
                exposed_ports,
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
                    &exposed_ports,
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
}
