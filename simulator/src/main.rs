use clap::Parser;
use cli::Cli;

mod cli;
mod config;
mod distributions;
mod node;
mod queue;
mod request;
mod system;

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    cli.run()
}
