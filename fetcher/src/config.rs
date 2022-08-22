use cosmoscout_models::config::DBConfig;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub fetcher: FetcherConfig,
    pub chain: ChainConfig,
    pub db: DBConfig,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ChainConfig {
    pub chain_id: String,
    pub chain_name: String,
    pub icon_url: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FetcherConfig {
    pub chain_id: String,
    pub tendermint_rpc: String,
    pub cosmos_grpc: String,
    pub start_block: u32,
    pub try_resume_from_db: bool,
}

impl Config {
    pub fn from_file(file: String) -> Result<Self, Box<dyn Error>> {
        let raw_config = fs::read_to_string(file)?;
        let config: Config = toml::from_str(raw_config.as_str()).unwrap();
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let config = Config::from_file("../config.toml".to_string()).unwrap();
        assert_eq!(
            config.fetcher,
            FetcherConfig {
                chain_id: "gaia".to_string(),
                tendermint_rpc: "http://localhost:26657/".to_string(),
                cosmos_grpc: "http://localhost:9090/".to_string(),
                start_block: 1,
                try_resume_from_db: true,
            }
        );
    }
}
