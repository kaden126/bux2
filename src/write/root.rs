use crate::read::Config;
use anyhow::Context;
use std::io::Write;

pub fn write_root(config: &Config) -> anyhow::Result<()> {
    let path = std::env::current_dir()
        .with_context(|| "Failed to retrieve working directory.")?
        .join("build")
        .join("root.build");
    
    let mut file = std::fs::File::create(&path)
        .with_context(|| format!("Failed to open {}.", path.display()))?;
    
    writeln!(&mut file, "using cxx")
        .with_context(|| format!("Failed to write {}.", path.display()))?;
    
    if let Some(extensions) = &config.extensions {
        writeln!(&mut file, "{extensions}")
            .with_context(|| format!("Failed to write {}.", path.display()))?;
    }
    
    Ok(())
}

pub fn write_profiles(config: &Config) -> anyhow::Result<()> {
    for (k, v) in &config.profile {
        let path = std::env::current_dir()
            .with_context(|| "Failed to retrieve working directory.")?
            .join("build")
            .join(format!("{k}.config"));
        
        let mut file = std::fs::File::create(&path)
            .with_context(|| format!("Failed to open {}.", path.display()))?;
        
        writeln!(&mut file, "{}", v)
            .with_context(|| format!("Failed to write {}.", path.display()))?;
    }
    
    Ok(())
}