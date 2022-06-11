#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env)]
    pub environment: String,
    #[clap(long, env)]
    pub database_url: String,
    #[clap(long, env)]
    pub rust_log: String,
    #[clap(long, env)]
    pub argon_salt: String,
    #[clap(long, env)]
    pub token_secret: String,
    #[clap(long, env)]
    pub port: u32,
    #[clap(long, env)]
    pub run_migrations: bool,
    #[clap(long, env)]
    pub seed: bool,
}

impl AppConfig {
    pub fn is_development(&self) -> bool {
        self.environment.eq("development")
    }

    pub fn is_production(&self) -> bool {
        self.environment.eq("production")
    }
}
