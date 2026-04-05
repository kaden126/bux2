use std::{fs::File, io::Read, collections::HashMap};

use anyhow::Context;
use serde::Deserialize;

pub mod package;
use package::Package;

pub mod dependencies;
use dependencies::Dependencies;

pub mod target;
use target::Target;

pub mod toolchain;
use toolchain::Toolchain;

pub mod extensions;
use extensions::Extensions;

pub mod test;
use test::Test;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub package: Package,
    
    pub dependencies: Option<Dependencies>,
    
    #[serde(rename = "build-dependencies")]
    pub build_dependencies: Option<Dependencies>,
    
    pub target: Vec<Target>,
    
    pub test: Option<Vec<Test>>,
    
    pub profile: HashMap<String, Toolchain>,
    
    pub extensions: Option<Extensions>
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let mut file = File::open("bux2.toml")
            .with_context(|| "Failed to open ./build2.toml")?;
        
        let mut content = String::new();
        file.read_to_string(&mut content)
            .with_context(|| "Failed to read from ./build2.toml")?;
        
        Ok(
            toml::from_slice(content.as_bytes())
                .with_context(|| "Failed to parse TOML.")?
        )
    }
}

#[cfg(test)]
mod tests {
    use toml::toml;
    use super::*;

    #[test]
    fn parse_package() {
        let toml = toml! {
            [package]
            name = "my-project"
            version = "0.1.0"
            summary = "testing for bux2"
            license = "MIT"

            keywords = ["testing", "validation"]
            categories = ["test the stuff!", "high quality testing"]
            readme = "README.md"

            homepage = "https://my-project-page.com"
            documentation = "https://my-project-docs.com"
            repository = "https://my-project-source.com"

            [extensions]
            sources = "cpp"
            header = "hpp"
            modules = "mpp"

            [profile.release]
            std = "latest"
            compiler = "clang++"

            coptions = ["-O2"]
            poptions = ["-DNDEBUG"]
            loptions = ["-flto=thin"]

            [dependencies]
            channels = ["stable", "testing", "legacy"]
            libremote = { version = "1.0.0", targets = ["remote"] }
            libpath = { version = "1.0.0", path = "/Users/me/libgoodbye", targets = ["path"] }
            libgit = { version = "1.0.0", repo = "https://me/libgit.git", targets = ["git"] }

            [[test]]
            description = "Test an important thing"
            src = "src"
            args = ["Hello", "World!"]

            stdout = "Hello"
            stderr = "World"
            stdin = "Input!"

            exit = "== 0"
        };
        Config::deserialize(toml).unwrap();
    }
}
