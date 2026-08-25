use clap::Parser;
use deaffiner::cli::{Cli, process_cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    process_cli(cli)
}
