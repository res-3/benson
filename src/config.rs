use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub heart_react_channel: u64,
    pub developer_channel: u64,
    pub guild: u64,
    pub bot_developer_role: u64,
}