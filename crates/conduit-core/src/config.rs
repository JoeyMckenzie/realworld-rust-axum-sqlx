#[derive(clap::Parser)]
pub struct AppConfig {
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
    #[clap(long, env)]
    pub cors_origin: String,
}
