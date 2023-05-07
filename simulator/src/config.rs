use config::File;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::distributions;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ConsumingDisrtibution {
    Exponential { lambda: f64 },
    Degenerate { mu: f64 },
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProducingDistribution {
    lambda: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProducerParams {
    pub(crate) producing_distribution: ProducingDistribution,
    pub(crate) consuming_distribution: ConsumingDisrtibution,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) nodes_number: usize,
    pub(crate) queue_capacity: usize,

    pub(crate) ticks: u64,

    #[serde(flatten)]
    pub(crate) producer: ProducerParams,

    pub(crate) output_file: String,
}

impl Config {
    ///! Loads config from file
    pub(crate) fn from_file(path: PathBuf) -> eyre::Result<Self> {
        Ok(config::Config::builder()
            .add_source(File::from(path))
            .build()
            .map_err(|e| eyre::eyre!("Failed to load config: {}", e))?
            .try_deserialize::<Self>()
            .map_err(|e| eyre::eyre!("Failed to parse config: {}", e))?)
    }
}

impl From<ProducingDistribution> for distributions::ProducingDistribution {
    fn from(value: ProducingDistribution) -> Self {
        match value {
            ProducingDistribution { lambda } => Self::Exponential { λ: lambda },
        }
    }
}

impl From<ConsumingDisrtibution> for distributions::ConsumingDistribution {
    fn from(value: ConsumingDisrtibution) -> Self {
        match value {
            ConsumingDisrtibution::Exponential { lambda } => Self::Exponential { λ: lambda },
            ConsumingDisrtibution::Degenerate { mu } => Self::Degenerate { μ: mu },
        }
    }
}
