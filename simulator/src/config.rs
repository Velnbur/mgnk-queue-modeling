use config::File;
use queuing_system_modeling::distributions;
use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ConsumingDisrtibution {
    Exponential { expected: f64 },
    Degenerate { expected: f64 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ProducingDistribution {
    expected: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct ProducerParams {
    pub(crate) producing_distribution: ProducingDistribution,
    pub(crate) consuming_distribution: ConsumingDisrtibution,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Experiment {
    pub(crate) nodes_number: usize,
    pub(crate) queue_capacity: usize,

    pub(crate) seconds: f64,

    #[serde(flatten)]
    pub(crate) producer: ProducerParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) experiments: HashMap<String, Experiment>,

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
            // Exponential distribution is parametrized by λ, but we have expected value.
            // So we need to convert it to λ
            ProducingDistribution { expected } => Self::Exponential { λ: 1.0 / expected },
        }
    }
}

impl From<ConsumingDisrtibution> for distributions::ConsumingDistribution {
    fn from(value: ConsumingDisrtibution) -> Self {
        match value {
            ConsumingDisrtibution::Exponential { expected } => {
                Self::Exponential { λ: 1.0 / expected }
            }
            ConsumingDisrtibution::Degenerate { expected } => {
                Self::Degenerate { μ: 1.0 / expected }
            }
        }
    }
}
