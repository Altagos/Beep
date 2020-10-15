use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub bot: BotConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BotConfig {
    pub default_prefix: String,
    pub invite_url: Option<String>,
    pub support_server_id: Option<u64>,
    pub token: String,
}
