use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub heart_react_channel: u64,
    pub developer_channel: u64,
    pub guild: u64,
    pub bot_developer_role: u64,
    pub benson_responses: Vec<String>,
    pub sentry_ingest_url: String
}