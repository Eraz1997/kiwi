use clap::Parser;

#[derive(Parser, Debug)]
pub struct Settings {
    #[arg(long, default_value = "3000")]
    pub dev_frontend_server_port: i32,
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value = "info")]
    pub log_level: tracing::Level,
    #[arg(long, default_value = "5000")]
    port: i32,
}

impl Settings {
    pub fn connection_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn is_development(&self) -> bool {
        cfg!(debug_assertions)
    }

    pub fn get_home_dir(&self) -> String {
        dirs::home_dir()
            .and_then(|directory| directory.into_os_string().into_string().ok())
            .unwrap_or_default()
    }
}
