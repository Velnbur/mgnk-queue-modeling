use std::sync::mpsc::channel;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use queuing_system_modeling::system::System;
use threadpool::ThreadPool;

use crate::{
    broadcaster,
    config::{Config, Experiment},
    stats::SysState,
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
pub(crate) fn run_simulation(
    name: String,
    config: Experiment,
    pb: ProgressBar,
    _stop_rx: broadcaster::Receiver<()>,
) {
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
    let mut current_time = 0.0;

    let mut wrt = csv::Writer::from_path(format!("{}.csv", name)).unwrap();

    wrt.write_record(&[
        "seconds",
        "requests_in_system",
        "waiting_mean",
        "reqs_in_system_mean",
    ])
    .unwrap();

    let mut last_state = SysState::default();

    while current_time < seconds {
        let state = system.next();
        current_time = state.current_tick;

        last_state.next(
            current_time,
            state.requests_in_system,
            state.finished_request,
        );

        pb.set_position(current_time as u64);

        wrt.write_record(last_state.to_strings()).unwrap();
    }
    pb.finish();
}

///! Run multiple simulation in parallel
pub(crate) fn run_simulations(config: Config) {
    let (mut stop_tx, _stop_rx) = broadcaster::channel();

    let num_thread = num_cpus::get();
    let pool = ThreadPool::new(num_thread);
    let (tx, rx) = channel();
    let experiments_number = config.experiments.len();
    let m = MultiProgress::new();

    let mut sorted: Vec<(String, Experiment)> = config.experiments.into_iter().collect();
    sorted.sort_by(|(_, a), (_, b)| a.seconds.partial_cmp(&b.seconds).unwrap().reverse());

    for (i, (desc, experiment)) in sorted.into_iter().enumerate() {
        let config = experiment.clone();
        let tx = tx.clone();
        let stop_rx = stop_tx.subscribe();

        let pb = m.insert(i, ProgressBar::new(config.seconds as u64));
        pb.set_style(PROGRESS_BAR_STYLE.clone());

        pool.execute(move || {
            tx.send(run_simulation(desc, config, pb, stop_rx))
                .expect("channel will be there waiting for the pool");
        });
    }

    // ctrlc::set_handler(move || {
    //     stop_tx.send(()).expect("Failed to send stop signal");
    // })
    // .expect("Error setting Ctrl-C handler");

    let _ = rx.iter().take(experiments_number).collect::<Vec<()>>();
}

// /// Convert results to csv
// pub(crate) fn convert_to_csv(
//     input: PathBuf,
//     output_stats: PathBuf,
//     output_dstr: PathBuf,
// ) -> eyre::Result<()> {
//     let input = std::fs::File::open(input).context("Failed to open input file")?;

//     // let mut results: Results =
//     //     serde_json::from_reader(input).context("Failed to parse input file")?;

//     results
//         .0
//         .sort_by(|a, b| a.seconds.partial_cmp(&b.seconds).unwrap());

//     convert_stats_to_csv(&results, output_stats)?;
//     convert_dstr_to_csv(&results, output_dstr)?;

//     Ok(())
// }

// fn convert_stats_to_csv(results: &Results, output: PathBuf) -> eyre::Result<()> {
//     let mut wtr = csv::Writer::from_path(output).context("Failed to open output file")?;

//     // Write headers
//     wtr.write_record(&["seconds", "waiting_mean", "queue_length_mean"])
//         .context("Failed to write headers")?;

//     for result in results.0.iter() {
//         wtr.write_record(&[
//             result.seconds.to_string(),
//             result.waiting_mean.to_string(),
//             result.queue_length_mean.to_string(),
//         ])?;
//     }

//     Ok(())
// }

// ///! Convert distribution to csv
// fn convert_dstr_to_csv(results: &Results, output: PathBuf) -> eyre::Result<()> {
//     if results.0.is_empty() {
//         return Err(eyre::eyre!("No results"));
//     }

//     let mut wtr = csv::Writer::from_path(output).context("Failed to open output file")?;

//     // Write headers
//     let headers = (0..results.0[0].p_k.len())
//         .map(|i| format!("p_{}", i))
//         .collect::<Vec<_>>();

//     wtr.write_record(&headers)
//         .context("Failed to write headers")?;

//     for result in results.0.iter() {
//         wtr.write_record(result.p_k.iter().map(|p| p.to_string()).collect::<Vec<_>>())?;
//     }

//     Ok(())
// }
