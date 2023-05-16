use queuing_system_modeling::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(crate) struct SysState {
    pub(crate) time: f64,
    pub(crate) requests_in_system: usize,
    pub(crate) reqs_in_system_mean: f64,
    pub(crate) waiting_mean: f64,

    iterations: usize,
    finished_requests: usize,
}

impl SysState {
    pub(crate) fn next(
        &mut self,
        seconds: f64,
        requests_in_system: usize,
        finished_request: Option<Request>,
    ) {
        if let Some(req) = finished_request {
            self.waiting_mean =
                Self::calc_waiting_mean(self.waiting_mean, req, self.finished_requests);
            self.finished_requests += 1;
        }
        self.reqs_in_system_mean = Self::calc_queue_length_mean(
            self.reqs_in_system_mean,
            requests_in_system,
            self.iterations,
        );
        self.requests_in_system = requests_in_system;

        self.iterations += 1;

        self.time = seconds;
    }

    pub(crate) fn to_strings(&self) -> [String; 4] {
        [
            self.time.to_string(),
            self.requests_in_system.to_string(),
            self.waiting_mean.to_string(),
            self.reqs_in_system_mean.to_string(),
        ]
    }

    fn calc_waiting_mean(
        last_waiting_mean: f64,
        request: Request,
        finished_requests_num: usize,
    ) -> f64 {
        let last_sum = last_waiting_mean * (finished_requests_num as f64);

        let waiting_time = request.started_at.unwrap() - request.created_at.unwrap();

        (last_sum + waiting_time) / ((finished_requests_num + 1) as f64)
    }

    fn calc_queue_length_mean(last: f64, current: usize, iterations: usize) -> f64 {
        (last * (iterations as f64) + (current as f64)) / ((iterations + 1) as f64)
    }
}
