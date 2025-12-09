use crate::domain::Pager;
use anyhow::Context;

use crate::repository::{DbClient, Neo4jClient, Neo4jConfig, NeptuneClient};
use aws_config::BehaviorVersion;
use aws_sdk_neptunedata::config::ProvideCredentials;

pub fn get_pager() -> anyhow::Result<Pager> {
    let pager_env_var = get_env_var("GRF_PAGER")?;
    let pager = match pager_env_var {
        Some(p) => Pager::custom(&p)?,
        None => Pager::default()?,
    };

    Ok(pager)
}

pub fn get_mandatory_env_var(key: &str) -> anyhow::Result<String> {
    get_env_var(key)?.context(format!("{} is not set", key))
}

pub fn get_env_var(key: &str) -> anyhow::Result<Option<String>> {
    match std::env::var(key) {
        Ok(v) => Ok(Some(v)),
        Err(e) => match e {
            std::env::VarError::NotPresent => Ok(None),
            std::env::VarError::NotUnicode(_) => anyhow::bail!("{} is not valid unicode", key),
        },
    }
}

#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub enum DbClientError {
    #[error("DB_URI is not set")]
    DBUriNotSet,
}

pub async fn get_db_client() -> anyhow::Result<DbClient> {
    let db_uri = get_mandatory_env_var("DB_URI")?;

    let db_client = match db_uri.split_once("://") {
        Some(("http", _)) | Some(("https", _)) => {
            let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
            if let Some(provider) = sdk_config.credentials_provider() {
                provider
                    .provide_credentials()
                    .await
                    .context("couldn't fetch AWS credentials")?;
            }

            let neptune_client = NeptuneClient::new(&sdk_config, &db_uri);
            DbClient::Neptune(neptune_client)
        }
        Some(("bolt", _)) => {
            let user = get_mandatory_env_var("NEO4J_USER")?;
            let password = get_mandatory_env_var("NEO4J_PASSWORD")?;
            let database_name = get_mandatory_env_var("NEO4J_DB")?;

            let config = Neo4jConfig {
                db_uri,
                user,
                password,
                database_name,
            };

            let neo4j_client = Neo4jClient::new(&config).await?;
            DbClient::Neo4j(neo4j_client)
        }
        Some((_, _)) => {
            anyhow::bail!("db uri must have one of the following protocols: [http, https, bolt]")
        }
        None => anyhow::bail!(
            r#"db uri must be a valid uri, eg. "bolt://127.0.0.1:7687", or "https://abc.xyz.us-east-1.neptune.amazonaws.com:8182""#
        ),
    };

    Ok(db_client)
}
