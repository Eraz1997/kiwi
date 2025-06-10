use tokio_postgres::Row;

use crate::managers::db::error::Error;

pub struct UserData {
    pub id: u32,
    pub password_hash: String,
}

impl TryFrom<Row> for UserData {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get("id")?,
            password_hash: value.try_get("password_hash")?,
        })
    }
}
