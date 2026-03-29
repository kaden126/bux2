use crate::read::{Config, dependencies::{Channel, DependencyInfo}};
use anyhow::Context;
use std::io::Write;

pub fn write_repos(config: &Config) -> anyhow::Result<()> {
    let path = std::env::current_dir()
        .with_context(|| "Failed to retrieve working directory.")?
        .join("repositories.manifest");
    
    let mut file = std::fs::File::create(&path)
        .with_context(|| format!("Failed to open {}", path.display()))?;
    
    writeln!(&mut file, ": 1")
        .with_context(|| format!("Failed to write {}", path.display()))?;
    
    if let Some(dependencies) = &config.dependencies {
        if let Some(channels) = &dependencies.channels {
            for channel in channels {
                writeln!(&file, "role: prerequisite")
                    .with_context(|| format!("Failed to write {}", path.display()))?;
                
                let url = match channel {
                    Channel::Stable => "https://pkg.cppget.org/1/stable",
                    Channel::Testing => "https://pkg.cppget.org/1/testing",
                    Channel::Legacy => "https://pkg.cppget.org/1/legacy",
                    Channel::Alpha => "https://pkg.cppget.org/1/alpha",
                    Channel::Beta => "https://pkg.cppget.org/1/beta"
                };
                
                writeln!(&file, "location: {url}")
                    .with_context(|| format!("Failed to write {}", path.display()))?;
            }
        };
        
        if let Some(deps) = &dependencies.deps {
            for (_, v) in deps {
                match &v.info {
                    DependencyInfo::LocalDependency(dep) => {
                        writeln!(&mut file, "role: prerequisite\nlocation: {}", dep.path.display())
                            .with_context(|| "Failed to write")?;
                    },
                    DependencyInfo::GitDependency(dep) => {
                        writeln!(&mut file, "role: prerequisite\nlocation: {}", dep.repo)
                            .with_context(|| "Failed to write")?;
                    },
                    _ => ()
                }
            }
        }
    }
    
    Ok(())
}