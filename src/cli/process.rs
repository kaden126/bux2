use anyhow::{Context, anyhow};
use std::fs::{File, create_dir};
use std::{io::Write, process::Command};

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

pub fn ci(profile: &str) -> anyhow::Result<()> {
    crate::generate_build_files().with_context(|| "Failed to generate build files.")?;

    let output = Command::new("bdep")
        .arg("ci")
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

pub fn publish(profile: &str) -> anyhow::Result<()> {
    crate::generate_build_files().with_context(|| "Failed to generate build files.")?;

    let output = Command::new("bdep")
        .arg("publish")
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

pub fn new(name: &str) -> anyhow::Result<()> {
    let root = std::env::current_dir()
        .with_context(|| "Failed to retrieve working directory.")?
        .join(name);
    create_dir(&root)
        .with_context(|| format!("Failed to create project directory {}", root.display()))?;

    let build = root.join("build");
    create_dir(&build)
        .with_context(|| format!("Failed to create build directory {}", build.display()))?;

    let source = root.join(&name);
    create_dir(&source)
        .with_context(|| format!("Failed to create source directory {}", source.display()))?;

    let config = root.join("bux2.toml");
    let mut file = File::create(&config)
        .with_context(|| format!("Failed to create config file {}", config.display()))?;

    writeln!(
        &mut file,
        r#"[package]
name = "{name}"
version = "0.1.0"
summary = "A package built with build2 + bux2!"
license = "MIT"

[[target]]
type = "exe"
name = "{name}"
src = "{name}"

[extensions]
sources = "cpp"
headers = "hpp"
modules = "mpp"

[dependencies]
# https://cppget.org repositories to fetch from.
# stable, testing, alpha, beta, and legacy are available.
# channels = ["stable"]
# libhello = {{ version = "1.0.0", targets = ["hello"] }}

[profile.debug]
# Select your debug configurations
# std = "latest"
# compiler = "clang++"
# coptions = ["-g"]
# poptions = []
# loptions = []
# Uncomment to enable C++ modules (C++20)
# modules = true

[profile.release]
# Select your release configurations
# std = "latest"
# compiler = "clang++"
# coptions = ["-O2"]
# poptions = ["-DNDEBUG"]
# loptions = []
# Uncomment to enable C++ modules (C++20)
# modules = true"#
    )
    .with_context(|| format!("Failed to write to config file {}", config.display()))?;

    let src = source.join("main.cpp");
    let mut source_file = File::create(&src)
        .with_context(|| format!("Failed to create source file {}", config.display()))?;

    writeln!(
        &mut source_file,
        r#"#include <iostream>

int main(int argc, char** argv) {{
    std::cout << "Hello, World!\n";
}}"#
    )
    .with_context(|| format!("Failed to write to source file {}.", source.display()))?;

    Ok(())
}
