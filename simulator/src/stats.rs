use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ExpResult {
    pub(crate) desciption: String,
    pub(crate) waiting_mean: f64,
    pub(crate) queue_length_mean: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Results(pub Vec<ExpResult>);
