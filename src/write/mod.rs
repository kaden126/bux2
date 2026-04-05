pub mod bootstrap;
pub mod buildfile;
pub mod manifest;
pub mod repos;
pub mod root;
pub mod testscript;

use crate::read::Config;
use anyhow::Context;

pub fn write(config: &Config) -> anyhow::Result<()> {
    root::write_root(config).with_context(|| "Writing to root.build failed.")?;

    root::write_profiles(config).with_context(|| "Writing profile config files failed.")?;

    bootstrap::write_bootstrap(config).with_context(|| "Writing to bootstrap.build failed.")?;

    buildfile::write_top_buildfile().with_context(|| "Writing to root buildfile failed.")?;

    buildfile::write_buildfiles(config).with_context(|| "Writing to buildfiles failed.")?;

    manifest::write_manifest(config).with_context(|| "Writing to manifest failed.")?;

    repos::write_repos(config).with_context(|| "Writing to manifest failed.")?;

    testscript::write_testscripts(config).with_context(|| "Writing to testscripts failed.")?;

    Ok(())
}
