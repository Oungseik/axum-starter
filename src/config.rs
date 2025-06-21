use clap::Parser;
use std::sync::OnceLock;

#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env)]
    pub database_url: String,
    #[clap(long, env, default_value = "8989")]
    pub port: u16,
}

pub fn get_config() -> &'static Config {
    dotenv::dotenv().ok();
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(Config::parse)
}
