use std::{fs::File, path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
use console::{style, Emoji};
use eyre::Context;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{config::Config, system::System};

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    #[arg(short = 'c', default_value = "PathBuf::from(\"./config.toml\")")]
    pub(crate) config: PathBuf,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Run,
}

static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "->");
static FILE: Emoji<'_, '_> = Emoji("🗄", "->");

impl Cli {
    pub(crate) fn run(self) -> eyre::Result<()> {
        let config = Config::from_file(self.config)?;

        match self.command {
            Commands::Run => {
                let pb = ProgressBar::new(config.ticks as u64);

                let mut system = System::new(
                    config.nodes_number,
                    config.queue_capacity,
                    config.producer.consuming_distribution.into(),
                    config.producer.producing_distribution.into(),
                );
                let mut states = Vec::with_capacity(config.ticks as usize);

                println!(
                    "{} {}Running simulation...",
                    style("[1/2]").bold().dim(),
                    TRUCK
                );

                for _ in 0..=config.ticks {
                    let state = system.tick();
                    states.push(state);

                    pb.inc(1);
                }

                pb.finish();

                let pb = ProgressBar::new_spinner();

                pb.enable_steady_tick(Duration::from_millis(120));
                pb.set_style(
                    ProgressStyle::with_template("{msg} {spinner:.blue} ")
                        .unwrap()
                        .tick_strings(&[".  ", ".. ", "...", " ..", "  .", "   "]),
                );
                pb.set_message(format!(
                    "{} {} Writing results to file",
                    style("[2/2]").bold().dim(),
                    FILE
                ));

                let file =
                    File::create(config.output_file).context("Failed to open `output_file`")?;
                serde_json::to_writer(file, &states).context("Failed to write to `output_file`")?;
            }
        }
        Ok(())
    }
}
