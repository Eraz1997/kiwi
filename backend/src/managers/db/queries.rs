use crate::managers::db::{DbManager, error::Error, models::UserData};

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
}
