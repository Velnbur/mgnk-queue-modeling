use queuing_system_modeling::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct SysState {
    pub(crate) time: f64,
    pub(crate) system_requests: usize,
    pub(crate) mean_queue_length: f64,
    pub(crate) mean_waiting_time: f64,
}

impl SysState {
    pub(crate) fn new(
        second_elapsed: f64,
        queue_length: usize,
        finished_requests: Vec<Request>,
        system_states: Vec<usize>,
    ) -> Self {
        let waiting_mean = Self::calc_waiting_mean(finished_requests);
        let queue_length_mean = Self::calc_queue_length_mean(system_states);

        Self {
            time: second_elapsed,
            system_requests: queue_length,
            mean_queue_length: queue_length_mean,
            mean_waiting_time: waiting_mean,
        }
    }

    fn calc_waiting_mean(finished_requests: Vec<Request>) -> f64 {
        let finished_requests_num = finished_requests.len();
        let sum: f64 = finished_requests
            .into_iter()
            .map(|r| (r.started_at.unwrap() - r.created_at.unwrap()).abs())
            .sum();
        sum / (finished_requests_num as f64)
    }

    fn _calc_p_k(p_k: Vec<usize>) -> Vec<f64> {
        let sum = p_k.iter().sum::<usize>();
        p_k.into_iter().map(|k| (k as f64) / (sum as f64)).collect()
    }

    fn calc_queue_length_mean(system_states: Vec<usize>) -> f64 {
        let queue_lengths_num = system_states.len();
        let sum = system_states.into_iter().sum::<usize>();
        (sum as f64) / (queue_lengths_num as f64)
    }
}
