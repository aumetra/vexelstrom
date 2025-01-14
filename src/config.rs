use serde::Deserialize;
use std::{net::IpAddr, num::NonZeroUsize, path::Path};

const MIN_RECOMMENDED_DB_CONNS: usize = 5;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DatabaseConfiguration {
    pub url: String,
    pub num_conns: NonZeroUsize,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfiguration {
    pub interface: IpAddr,
    pub port: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Configuration {
    pub database: DatabaseConfiguration,
    pub server: ServerConfiguration,
}

impl Configuration {
    #[instrument(skip_all)]
    pub fn lint(&self) {
        if self.database.num_conns.get() < MIN_RECOMMENDED_DB_CONNS {
            warn!(
                configured = self.database.num_conns.get(),
                recommended_min = MIN_RECOMMENDED_DB_CONNS,
                "less than the recommended minimum amount of database connections",
            );
        }
    }

    pub async fn load<P>(path: P) -> eyre::Result<Self>
    where
        P: AsRef<Path>,
    {
        let raw_config = tokio::fs::read_to_string(path).await?;
        let config: Self = toml::from_str(&raw_config)?;
        config.lint();

        Ok(config)
    }
}
