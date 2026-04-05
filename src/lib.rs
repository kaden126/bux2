pub mod cli;
pub mod read;
pub mod write;

use anyhow::Context;
use clap::Parser;
use cli::CliArgs;

pub fn generate_build_files() -> anyhow::Result<()> {
    let config = read::Config::new().with_context(|| "Failed to read config.")?;

    write::write(&config).with_context(|| "Failed to write build2 files.")?;

    Ok(())
}

pub fn parse_cli() -> CliArgs {
    CliArgs::parse()
}
