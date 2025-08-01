use uuid::Uuid;

use crate::error::Error;
use crate::managers::db::{
    DbManager,
    models::{UserData, UserInvitation},
};
use crate::models::UserRole;

impl DbManager {
    pub async fn get_user_data(&self, username: &String) -> Result<Option<UserData>, Error> {
        let client = self.connection_pool.get().await?;
        let statement = client
            .prepare_cached("SELECT * FROM users WHERE username = $1")
            .await?;
        let user_data: Option<UserData> = client
            .query_opt(&statement, &[username])
            .await?
            .map(UserData::try_from)
            .and_then(Result::ok);
        Ok(user_data)
    }

    pub async fn get_user_data_from_id(&self, id: &i64) -> Result<Option<UserData>, Error> {
        let client = self.connection_pool.get().await?;
        let statement = client
            .prepare_cached("SELECT * FROM users WHERE id = $1")
            .await?;
        let user_data: Option<UserData> = client
            .query_opt(&statement, &[id])
            .await?
            .map(UserData::try_from)
            .and_then(Result::ok);
        Ok(user_data)
    }

    pub async fn get_users_data(&self) -> Result<Vec<UserData>, Error> {
        let client = self.connection_pool.get().await?;
        let statement = client.prepare_cached("SELECT * FROM users").await?;
        let users: Result<Vec<UserData>, Error> = client
            .query(&statement, &[])
            .await?
            .into_iter()
            .map(UserData::try_from)
            .collect();
        users
    }

    pub async fn delete_user(&self, username: &String) -> Result<(), Error> {
        let client = self.connection_pool.get().await?;
        let statement = client
            .prepare_cached("DELETE FROM users WHERE username = $1")
            .await?;
        client.execute(&statement, &[username]).await?;
        Ok(())
    }

    pub async fn get_or_create_admin_invitation_if_no_admin_yet(
        &self,
    ) -> Result<Option<UserInvitation>, Error> {
        let mut client = self.connection_pool.get().await?;
        let transaction = client.transaction().await?;

        let statement = transaction
            .prepare_cached("SELECT * FROM user_invitations WHERE role = 'Admin'")
            .await?;
        let existing_invitation: Option<UserInvitation> = transaction
            .query_opt(&statement, &[])
            .await?
            .map(UserInvitation::try_from)
            .and_then(Result::ok);
        let statement = transaction
            .prepare_cached("SELECT * FROM users WHERE role = 'Admin'")
            .await?;
        let user_data: Option<UserData> = transaction
            .query_opt(&statement, &[])
            .await?
            .map(UserData::try_from)
            .and_then(Result::ok);
        let invitation = match (existing_invitation, user_data) {
            (Some(existing_invitation), _) => Some(existing_invitation),
            (None, Some(_)) => None,
            (None, None) => {
                let statement = transaction
                    .prepare_cached(
                        "INSERT INTO user_invitations (role) VALUES ('Admin') RETURNING id, role",
                    )
                    .await?;
                let invitation_raw = transaction.query_one(&statement, &[]).await?;
                let invitation = UserInvitation::try_from(invitation_raw)?;
                Some(invitation)
            }
        };

        transaction.commit().await?;
        Ok(invitation)
    }

    pub async fn create_user_from_invitation(
        &self,
        invitation_id: &Uuid,
        username: &String,
        password_hash: &String,
    ) -> Result<Option<UserData>, Error> {
        let mut client = self.connection_pool.get().await?;
        let transaction = client.transaction().await?;
        let statement = transaction
            .prepare_cached("SELECT id, role FROM user_invitations WHERE id = $1")
            .await?;
        let invitation: Option<UserInvitation> = transaction
            .query_opt(&statement, &[&invitation_id])
            .await?
            .map(UserInvitation::try_from)
            .and_then(Result::ok);
        let user_data = match invitation {
            None => None,
            Some(invitation) => {
                let statement = transaction.prepare_cached("INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3) RETURNING id, password_hash, role, username").await?;
                let user_data_raw = transaction
                    .query_one(&statement, &[&username, &password_hash, &invitation.role])
                    .await?;
                let user_data = UserData::try_from(user_data_raw)?;
                let statement = transaction
                    .prepare_cached("DELETE FROM user_invitations WHERE id = $1")
                    .await?;
                transaction.execute(&statement, &[&invitation_id]).await?;

                Some(user_data)
            }
        };
        transaction.commit().await?;
        Ok(user_data)
    }

    pub async fn create_user_invitation(&self, role: UserRole) -> Result<UserInvitation, Error> {
        let client = self.connection_pool.get().await?;
        let statement = client
            .prepare_cached("INSERT INTO user_invitations (role) VALUES ($1) RETURNING id, role")
            .await?;
        let invitation_raw = client.query_one(&statement, &[&role]).await?;
        let invitation = UserInvitation::try_from(invitation_raw)?;
        Ok(invitation)
    }
}
