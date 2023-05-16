use std::{fs::File, path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
use console::{style, Emoji};
use eyre::Context;
use indicatif::{ProgressBar, ProgressStyle};
use simplelog::{CombinedLogger, LevelFilter, WriteLogger};

use crate::{
    actions::{self},
    config::Config,
};

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    /// Use debug mode. (Will write logs to `debug.log`).
    #[arg(short = 'd')]
    pub(crate) debug_mode: bool,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Run simulations with the given config file.
    Run {
        /// Path to config file
        #[arg(short = 'c', default_value = "PathBuf::from(\"./config.toml\")")]
        config: PathBuf,
    },
    /// Convert results from json to csv
    Convert {
        /// Path to file with results
        #[arg(short = 'i', default_value = "PathBuf::from(\"./results.json\")")]
        input: PathBuf,

        /// Path to output stats file
        #[arg(short = 'o', default_value = "PathBuf::from(\"./stats.csv\")")]
        output_stats: PathBuf,

        /// Path to output distributions file
        #[arg(short = 'd', default_value = "PathBuf::from(\"./distributions.csv\")")]
        output_distributions: PathBuf,
    },
}

static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "->");
static FILE: Emoji<'_, '_> = Emoji("🗄", "->");

const LOG_FILE: &str = "debug.log";

impl Cli {
    pub(crate) fn run(self) -> eyre::Result<()> {
        if self.debug_mode {
            CombinedLogger::init(vec![WriteLogger::new(
                LevelFilter::Debug,
                simplelog::Config::default(),
                File::create(LOG_FILE)
                    .map_err(|err| eyre::eyre!("Failed to open file {LOG_FILE}: {err}"))?,
            )])
            .expect("Logger should always be initialized");
        }

        match self.command {
            Commands::Run {
                config: config_path,
            } => {
                let config = Config::from_file(config_path)?;
                println!(
                    "{} {}Running simulations...",
                    style("[1/2]").bold().dim(),
                    TRUCK
                );

                let output_file = config.output_file.clone();

                let results = actions::run_simulations(config);

                let pb = ProgressBar::new_spinner();
                pb.enable_steady_tick(Duration::from_millis(120));
                pb.set_style(
                    ProgressStyle::with_template("{msg} {spinner:.blue} ")
                        .unwrap()
                        .tick_strings(&[".  ", ".. ", "...", " ..", "  .", "   "]),
                );

                println!("{} {}Writing results...", style("[2/2]").bold().dim(), FILE);

                let file = File::create(output_file.clone())
                    .context(format!("Failed to open/create file: {output_file}"))?;

                serde_json::to_writer(file, &results)
                    .context(format!("Failed to write results to {output_file}"))?;
            }
            Commands::Convert {
                input: input_path,
                output_stats: output_stats_path,
                output_distributions: output_distributions_path,
            } => {
                unimplemented!()
                // convert_to_csv(input_path, output_stats_path, output_distributions_path)
                //     .context("Failed to convert to csv")?;
            }
        }
        Ok(())
    }
}
