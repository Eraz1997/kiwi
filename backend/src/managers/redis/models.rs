use fred::types::Expiration;
use time::Duration;

pub trait RedisItem {
    fn get_key_suffix(&self) -> String;
    fn get_value(&self) -> String;

    fn get_key(&self) -> String {
        format!("kiwi_admin:{}", self.get_key_suffix())
    }

    fn get_expiration(&self) -> Option<Expiration> {
        None
    }
}

pub struct RedisAccessToken {
    pub access_token: String,
    pub user_id: u32,
}

impl RedisItem for RedisAccessToken {
    fn get_key_suffix(&self) -> String {
        format!("access_token:{}", self.access_token)
    }

    fn get_value(&self) -> String {
        self.user_id.to_string()
    }

    fn get_expiration(&self) -> Option<Expiration> {
        Some(Expiration::EX(Duration::minutes(15).whole_seconds()))
    }
}

pub struct RedisRefreshToken {
    pub refresh_token: String,
    pub user_id: u32,
}

impl RedisItem for RedisRefreshToken {
    fn get_key_suffix(&self) -> String {
        format!("refresh_token:{}", self.refresh_token)
    }

    fn get_value(&self) -> String {
        format!("active:{}", self.user_id)
    }

    fn get_expiration(&self) -> Option<Expiration> {
        Some(Expiration::EX(Duration::days(14).whole_seconds()))
    }
}
