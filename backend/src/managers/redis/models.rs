use std::str::FromStr;

use fred::types::Expiration;
use time::Duration;

use crate::{error::Error, models::UserRole};

pub trait RedisItem: Sized {
    fn to_redis_key_suffix(&self) -> String;
    fn to_redis_value(&self) -> String;
    fn from_redis_key_suffix_and_value(key_suffix: String, value: String) -> Result<Self, Error>;

    fn to_redis_key(&self) -> String {
        format!("kiwi_admin:{}", self.to_redis_key_suffix())
    }

    fn from_redis_item(key: String, value: String) -> Result<Self, Error> {
        let mut consumed_key = key.clone();
        if !consumed_key.starts_with("kiwi_admin:") {
            return Err(Error::serialisation());
        }
        consumed_key = consumed_key[11..].to_string();

        Self::from_redis_key_suffix_and_value(consumed_key, value)
    }

    fn get_expiration(&self) -> Option<Expiration> {
        None
    }
}

pub struct RedisAccessToken {
    pub access_token: String,
    pub user_id: i64,
    pub sealing_key: String,
    pub role: UserRole,
}

impl RedisItem for RedisAccessToken {
    fn to_redis_key_suffix(&self) -> String {
        format!("access_token:{}", self.access_token)
    }

    fn to_redis_value(&self) -> String {
        format!("{}:{}:{}", self.user_id, self.sealing_key, self.role)
    }

    fn get_expiration(&self) -> Option<Expiration> {
        Some(Expiration::EX(Duration::minutes(15).whole_seconds()))
    }

    fn from_redis_key_suffix_and_value(key_suffix: String, value: String) -> Result<Self, Error> {
        let mut consumed_key = key_suffix.clone();
        if !consumed_key.starts_with("access_token:") {
            return Err(Error::serialisation());
        }
        consumed_key = consumed_key[13..].to_string();

        let values: Vec<String> = value.split(":").map(|value| value.to_string()).collect();

        let user_id: i64 = values
            .first()
            .ok_or(Error::serialisation())?
            .parse()
            .map_err(|_| Error::serialisation())?;
        let sealing_key = values.get(1).ok_or(Error::serialisation())?.clone();
        let role_raw = values.get(2).ok_or(Error::serialisation())?.clone();
        let role = UserRole::from_str(&role_raw)?;

        Ok(RedisAccessToken {
            access_token: consumed_key,
            user_id,
            sealing_key,
            role,
        })
    }
}

pub struct RedisActiveRefreshToken {
    pub user_id: i64,
    pub sealing_key: String,
    pub role: UserRole,
}

pub struct RedisRefreshedRefreshToken {
    pub fresh_refresh_token: String,
    pub fresh_access_token: String,
}

pub enum RedisRefreshTokenKind {
    Active(RedisActiveRefreshToken),
    Refreshed(RedisRefreshedRefreshToken),
}

pub struct RedisRefreshToken {
    pub refresh_token: String,
    pub kind: RedisRefreshTokenKind,
}

impl RedisItem for RedisRefreshToken {
    fn to_redis_key_suffix(&self) -> String {
        format!("refresh_token:{}", self.refresh_token)
    }

    fn to_redis_value(&self) -> String {
        match &self.kind {
            RedisRefreshTokenKind::Active(data) => {
                format!("active:{}:{}:{}", data.user_id, data.sealing_key, data.role,)
            }
            RedisRefreshTokenKind::Refreshed(data) => format!(
                "refreshed:{}:{}",
                data.fresh_access_token, data.fresh_refresh_token,
            ),
        }
    }

    fn get_expiration(&self) -> Option<Expiration> {
        match &self.kind {
            RedisRefreshTokenKind::Active(_) => {
                Some(Expiration::EX(Duration::days(14).whole_seconds()))
            }
            RedisRefreshTokenKind::Refreshed(_) => {
                Some(Expiration::EX(Duration::minutes(2).whole_seconds()))
            }
        }
    }

    fn from_redis_key_suffix_and_value(key_suffix: String, value: String) -> Result<Self, Error> {
        let mut consumed_key = key_suffix.clone();
        if !consumed_key.starts_with("refresh_token:") {
            return Err(Error::serialisation());
        }
        consumed_key = consumed_key[14..].to_string();

        let values: Vec<String> = value.split(":").map(|value| value.to_string()).collect();

        match values[0].as_str() {
            "active" => {
                let raw_user_id = values.get(1).ok_or(Error::serialisation())?;
                let user_id: i64 = raw_user_id.parse().map_err(|_| Error::serialisation())?;
                let sealing_key = values.get(2).ok_or(Error::serialisation())?.to_owned();
                let role_raw = values.get(2).ok_or(Error::serialisation())?.clone();
                let role = UserRole::from_str(&role_raw)?;
                Ok(RedisRefreshToken {
                    refresh_token: consumed_key,
                    kind: RedisRefreshTokenKind::Active(RedisActiveRefreshToken {
                        user_id,
                        sealing_key,
                        role,
                    }),
                })
            }
            "refreshed" => {
                let fresh_access_token = values.get(1).ok_or(Error::serialisation())?;
                let fresh_refresh_token = values.get(2).ok_or(Error::serialisation())?;
                Ok(RedisRefreshToken {
                    refresh_token: consumed_key,
                    kind: RedisRefreshTokenKind::Refreshed(RedisRefreshedRefreshToken {
                        fresh_access_token: fresh_access_token.to_string(),
                        fresh_refresh_token: fresh_refresh_token.to_string(),
                    }),
                })
            }
            _ => Err(Error::serialisation()),
        }
    }
}
