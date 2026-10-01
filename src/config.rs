use serde::{Deserialize, Serialize};

use crate::boundary::{identifier, reference, reject_secrets, secret_ref};
use crate::{CONFIG_SCHEMA, ConnectorError};

const DATA_SETS: &[&str] = &[
    "audience-insights",
    "delivery-statistics",
    "webhook-event-metadata",
];

/// Closed configuration for metadata-only LINE observations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectorConfig {
    pub schema: String,
    pub config_id: String,
    pub channel_ref: String,
    pub secret_ref: String,
    pub data_sets: Vec<String>,
}

impl ConnectorConfig {
    /// Enforces the allowlist that excludes messages and user identifiers.
    pub fn validate(&self) -> Result<(), ConnectorError> {
        if self.schema != CONFIG_SCHEMA {
            return Err(ConnectorError::new(
                "schema",
                "expected zixcel://line/config/v1",
            ));
        }
        identifier("config_id", &self.config_id)?;
        reference("channel_ref", &self.channel_ref)?;
        secret_ref(&self.secret_ref)?;
        if self.data_sets.is_empty() || self.data_sets.len() > DATA_SETS.len() {
            return Err(ConnectorError::new(
                "data_sets",
                "must contain 1..=3 data sets",
            ));
        }
        if self
            .data_sets
            .iter()
            .any(|value| !DATA_SETS.contains(&value.as_str()))
        {
            return Err(ConnectorError::new(
                "data_sets",
                "contains an unsupported data set",
            ));
        }
        Ok(())
    }

    pub(crate) fn normalized(&self) -> Self {
        let mut value = self.clone();
        value.data_sets.sort();
        value.data_sets.dedup();
        value
    }
}

/// Parses a bounded, closed TOML configuration.
pub fn parse_config(source: &str) -> Result<ConnectorConfig, ConnectorError> {
    reject_secrets(source)?;
    let config: ConnectorConfig = toml::from_str(source).map_err(|_| {
        ConnectorError::new(
            "config",
            "invalid TOML or fields do not match the LINE v1 schema",
        )
    })?;
    config.validate()?;
    Ok(config)
}
