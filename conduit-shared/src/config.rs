#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub connection_string: String,
    #[clap(long, env)]
    pub argon_salt: String,
    #[clap(long, env)]
    pub token_secret: String,
    #[clap(long, env)]
    pub port: u32,
}
