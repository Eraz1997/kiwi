use tokio_postgres::Row;
use uuid::Uuid;

use crate::error::Error;
use crate::models::UserRole;

pub struct UserData {
    pub id: i64,
    pub password_hash: String,
    pub role: UserRole,
    pub username: String,
}

impl TryFrom<Row> for UserData {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get("id")?,
            password_hash: value.try_get("password_hash")?,
            role: value.try_get("role")?,
            username: value.try_get("username")?,
        })
    }
}

pub struct UserInvitation {
    pub id: Uuid,
    pub role: UserRole,
}

impl TryFrom<Row> for UserInvitation {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get("id")?,
            role: value.try_get("role")?,
        })
    }
}
