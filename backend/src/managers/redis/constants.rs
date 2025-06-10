use std::time::Duration;

pub static DSN: &str = "redis://localhost:6379";
pub static CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
