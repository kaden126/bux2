use std::io::Write;

use anyhow::Context;

pub fn generate_cli(name: &str) -> anyhow::Result<()> {
    let compiler = std::env::var("CXX")
        .unwrap_or("clang++".into());

    let config = format!(
r#"[package]
name = "{name}"
version = "0.1.0"
summary = "A CLI package built with build2 + bux2!"
license = "MIT"

[[target]]
type = "exe"
name = "{name}"
src = "src"

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
std = "latest"
compiler = "{compiler}"
# coptions = ["-g"]
# poptions = []
# loptions = []
# Uncomment to enable C++ modules (C++20)
# modules = true

[profile.release]
# Select your release configurations
std = "latest"
compiler = "{compiler}"
# coptions = ["-O2"]
# poptions = ["-DNDEBUG"]
# loptions = []
# Uncomment to enable C++ modules (C++20)
# modules = true"#);

    let source = format!(
r#"
#include <iostream>

int main(int argc, char** argv) {{
    if (argc < 2) {{
        std::cerr << "usage: {name} <NAME>\n";
    }}
}}
"#);

    let path = std::env::current_dir()
        .with_context(|| "Invalid working directory.")?
        .join(name);

    std::fs::create_dir(&path)
        .with_context(|| format!("Failed to create project directory {}", path.display()))?;

    let config_path = path.join("bux2.toml");

    let mut config_file = std::fs::File::create(&config_path)
        .with_context(|| format!("Failed to create config file {}", config_path.display()))?;

    writeln!(&mut config_file, "{config}")?;

    let source_path = path
        .join("src");
        
    std::fs::create_dir(&source_path)
        .with_context(|| format!("Failed to create source directory {}", source_path.display()))?;

    let main_path = source_path.join("main.cpp");

    let mut main_file = std::fs::File::create(&main_path)
        .with_context(|| format!("Failed to create source file {}", source_path.display()))?;

    writeln!(&mut main_file, "{source}")
        .with_context(|| format!("Failed to write to main.cpp {}", main_path.display()))?;

    Ok(())
}

pub fn generate_lib(name: &str) -> anyhow::Result<()> {
    let compiler = std::env::var("CXX")
        .unwrap_or("clang++".into());

    let config = format!(
r#"[package]
name = "{name}"
version = "0.1.0"
summary = "A library package built with build2 + bux2!"
license = "MIT"

[[target]]
type = "lib"
name = "{name}"
src = "src"

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
std = "latest"
compiler = "{compiler}"
# coptions = ["-g"]
# poptions = []
# loptions = []
# Uncomment to enable C++ modules (C++20)
# modules = true

[profile.release]
# Select your release configurations
std = "latest"
compiler = "{compiler}"
# coptions = ["-O2"]
# poptions = ["-DNDEBUG"]
# loptions = []
# Uncomment to enable C++ modules (C++20)
# modules = true"#);

    let header =
r#"
#pragma once
#ifndef LIBGREET_HPP
#define LIBGREET_HPP

#include <string>

void hello(const std::string& name);

#endif
"#;

    let source =
r#"
#include "greet.hpp"
#include <iostream>

void hello(const std::string& name) {
    std::cout << "Hello, " << name << '\n';
}
"#;

    // Config
    let path = std::env::current_dir()
        .with_context(|| "Invalid working directory.")?
        .join(name);

    std::fs::create_dir(&path)
        .with_context(|| format!("Failed to create project directory {}", path.display()))?;

    let config_path = path.join("bux2.toml");

    let mut config_file = std::fs::File::create(&config_path)
        .with_context(|| format!("Failed to create config file {}.", config_path.display()))?;

    writeln!(&mut config_file, "{config}")?;

    // Header
    let source_path = path
        .join("src");
        
    std::fs::create_dir(&source_path)
        .with_context(|| format!("Failed to create source directory {}", source_path.display()))?;

    let header_path = source_path.join("greet.hpp");

    let mut header_file = std::fs::File::create(&header_path)
        .with_context(|| format!("Failed to create header file {}", header_path.display()))?;

    writeln!(&mut header_file, "{header}")
        .with_context(|| format!("Failed to write to greet.hpp {}", header_path.display()))?;

    // Source
    let cpp_path = source_path.join("greet.cpp");

    let mut cpp_file = std::fs::File::create(&cpp_path)
        .with_context(|| format!("Failed to create source file {}", cpp_path.display()))?;

    writeln!(&mut cpp_file, "{source}")
        .with_context(|| format!("Failed to write to greet.hpp {}", cpp_path.display()))?;

    Ok(())
}