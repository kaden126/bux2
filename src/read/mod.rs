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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub package: Package,
    
    pub dependencies: Option<Dependencies>,
    
    #[serde(rename = "build-dependencies")]
    pub build_dependencies: Option<Dependencies>,
    
    pub target: Vec<Target>,
    
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
