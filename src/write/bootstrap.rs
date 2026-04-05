use anyhow::Context;

use crate::read::Config;
use std::io::Write;

pub fn write_bootstrap(config: &Config) -> anyhow::Result<()> {
    let path = std::env::current_dir()
        .with_context(|| "Failed to retrieve working directory.")?
        .join("build")
        .join("bootstrap.build");

    let mut file = std::fs::File::create(&path)
        .with_context(|| format!("Failed to open {}.", path.display()))?;

    writeln!(
        &mut file,
        "project = {}\nusing version\nusing config\nusing install\nusing dist\nusing test",
        config.package.name
    )
    .with_context(|| format!("Failed to write {}.", path.display()))?;

    Ok(())
}
