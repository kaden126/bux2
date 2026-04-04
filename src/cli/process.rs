use anyhow::{Context, anyhow};
use std::{process::Command};
use crate::cli::generate::*;

pub fn build(profile: &str) -> anyhow::Result<()> {
    crate::generate_build_files().with_context(|| "Failed to generate build files.")?;

    let output = Command::new("bdep")
        .arg("update")
        .arg(format!("@{profile}"))
        .arg("--progress")
        .spawn()
        .with_context(|| "Failed to spawn bdep.")?
        .wait_with_output()
        .with_context(|| "Failed to wait on bdep process.")?;

    if !output.status.success() {
        return Err(anyhow!("bdep exited with exit code: {}", output.status));
    }

    Ok(())
}

pub fn fetch(profile: &str) -> anyhow::Result<()> {
    crate::generate_build_files().with_context(|| "Failed to generate build files.")?;

    let output = Command::new("bdep")
        .arg("fetch")
        .arg(format!("@{profile}"))
        .spawn()
        .with_context(|| "Failed to spawn bdep.")?
        .wait_with_output()
        .with_context(|| "Failed to wait on bdep process.")?;

    if !output.status.success() {
        return Err(anyhow!("bdep exited with exit code: {}", output.status));
    }

    Ok(())
}

pub fn ci() -> anyhow::Result<()> {
    crate::generate_build_files().with_context(|| "Failed to generate build files.")?;

    let output = Command::new("bdep")
        .arg("ci")
        .spawn()
        .with_context(|| "Failed to spawn bdep.")?
        .wait_with_output()
        .with_context(|| "Failed to wait on bdep process.")?;

    if !output.status.success() {
        return Err(anyhow!("bdep exited with exit code: {}", output.status));
    }

    Ok(())
}

pub fn publish() -> anyhow::Result<()> {
    crate::generate_build_files().with_context(|| "Failed to generate build files.")?;

    let output = Command::new("bdep")
        .arg("publish")
        .spawn()
        .with_context(|| "Failed to spawn bdep.")?
        .wait_with_output()
        .with_context(|| "Failed to wait on bdep process.")?;

    if !output.status.success() {
        return Err(anyhow!("bdep exited with exit code: {}", output.status));
    }

    Ok(())
}

pub fn clean(profile: &str, recursive: bool) -> anyhow::Result<()> {
    crate::generate_build_files().with_context(|| "Failed to generate build files.")?;

    let mut output = Command::new("bdep");

    output.arg("clean").arg(format!("@{profile}"));

    if recursive {
        output.arg("--recursive");
    } else {
        output.arg("--immediate");
    }

    let result = output
        .spawn()
        .with_context(|| "Failed to spawn bdep.")?
        .wait_with_output()
        .with_context(|| "Failed to wait on bdep process.")?;

    if !result.status.success() {
        return Err(anyhow!("bdep exited with exit code: {}", result.status));
    }

    Ok(())
}

pub fn init(profile: &str) -> anyhow::Result<()> {
    crate::generate_build_files().with_context(|| "Failed to generate build files.")?;

    let output = Command::new("bdep")
        .arg("init")
        .arg("-C")
        .arg(format!("@{profile}"))
        .arg("cxx")
        .arg("--wipe")
        .arg("--options-file")
        .arg(format!("build/{profile}.config"))
        .spawn()
        .with_context(|| "Failed to spawn bdep.")?
        .wait_with_output()
        .with_context(|| "Failed to wait on bdep process.")?;

    if !output.status.success() {
        return Err(anyhow!("bdep exited with exit code: {}", output.status));
    }

    Ok(())
}

pub fn deinit(profile: &str) -> anyhow::Result<()> {
    crate::generate_build_files().with_context(|| "Failed to generate build files.")?;

    let output = Command::new("bdep")
        .arg("deinit")
        .arg(format!("@{profile}"))
        .spawn()
        .with_context(|| "Failed to spawn bdep.")?
        .wait_with_output()
        .with_context(|| "Failed to wait on bdep process.")?;

    if !output.status.success() {
        return Err(anyhow!("bdep exited with exit code: {}", output.status));
    }
    
    let output = Command::new("bdep")
        .arg("config")
        .arg("remove")
        .arg(format!("@{profile}"))
        .spawn()
        .with_context(|| "Failed to spawn bdep.")?
        .wait_with_output()
        .with_context(|| "Failed to wait on bdep process.")?;
    
    if !output.status.success() {
        return Err(anyhow!("bdep exited with exit code: {}", output.status));
    }

    Ok(())
}

pub fn new(name: &str, template: Option<String>) -> anyhow::Result<()> {
    if let Some(template) = template {
        if template == "exe" {
            generate_cli(name)
        }
        else if template == "lib" {
            generate_lib(name)
        }
        else {
            anyhow::bail!("No such template: {template}\nExpected `lib` or `exe`")
        }
    }
    else {
        generate_cli(name)
    }
}
