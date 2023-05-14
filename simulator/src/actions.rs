use std::sync::mpsc::channel;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use queuing_system_modeling::system::{Stats, System};
use threadpool::ThreadPool;

use crate::{
    config::{Config, Experiment},
    stats::{ExpResult, Results},
};

static PROGRESS_BAR_TEMPLATE: &str =
    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}";
static PROGRESS_BAR_CHARS: &str = "#>-";

static PROGRESS_BAR_STYLE: Lazy<ProgressStyle> = Lazy::new(|| {
    ProgressStyle::with_template(PROGRESS_BAR_TEMPLATE)
        .unwrap()
        .progress_chars(PROGRESS_BAR_CHARS)
});

///! Run simulation with given config
pub(crate) fn run_simulation(name: String, config: Experiment, pb: ProgressBar) -> ExpResult {
    let Experiment {
        nodes_number,
        queue_capacity,
        producer,
        seconds,
        ..
    } = config;

    let mut system = System::new(
        nodes_number,
        queue_capacity,
        producer.consuming_distribution.into(),
        producer.producing_distribution.into(),
    );

    let mut last_state = Stats::default();
    let mut queue_lengths = Vec::new();

    while last_state.current_tick < seconds {
        last_state = system.next();
        queue_lengths.push(last_state.requests_in_system);
        pb.inc(1);
    }

    let finished_requests_num = last_state.finished_requests.len();
    let sum: f64 = last_state
        .finished_requests
        .into_iter()
        .map(|r| (r.started_at.unwrap() - r.created_at).abs())
        .sum();
    let waiting_mean = sum / (finished_requests_num as f64);

    let queue_lengths_num = queue_lengths.len();
    let sum = queue_lengths.into_iter().sum::<usize>();
    let queue_length_mean = (sum as f64) / (queue_lengths_num as f64);
    pb.finish();

    ExpResult {
        desciption: name,
        waiting_mean,
        queue_length_mean,
    }
}

///! Run multiple simulation in parallel
pub(crate) fn run_simulations(config: Config) -> Results {
    let num_thread = num_cpus::get();
    let pool = ThreadPool::new(num_thread);
    let (tx, rx) = channel();
    let experiments_number = config.experiments.len();
    let m = MultiProgress::new();

    for (i, (desc, experiment)) in config.experiments.into_iter().enumerate() {
        let config = experiment.clone();
        let tx = tx.clone();

        let pb = m.insert(i % num_thread, ProgressBar::new(config.seconds as u64));
        pb.set_style(PROGRESS_BAR_STYLE.clone());

        pool.execute(move || {
            tx.send(run_simulation(desc, config, pb))
                .expect("channel will be there waiting for the pool");
        });
    }

    let results: Vec<ExpResult> = rx.iter().take(experiments_number).collect();

    Results(results)
}
