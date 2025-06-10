use time::Duration;

pub static ACCESS_TOKEN_COOKIE_NAME: &str = "__kiwi_access_token";
pub static REFRESH_TOKEN_COOKIE_NAME: &str = "__kiwi_refresh_token";
pub static CREDENTIALS_DURATION: Duration = Duration::days(14);
