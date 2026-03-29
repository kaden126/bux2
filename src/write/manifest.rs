use crate::read::{Config, dependencies::{DependencyInfo}};
use anyhow::Context;
use std::io::Write;

pub fn write_manifest(config: &Config) -> anyhow::Result<()> {
    let path = std::env::current_dir()
        .with_context(|| "Failed to retrieve working directory.")?
        .join("manifest");
    
    let mut file = std::fs::File::create(&path)
        .with_context(|| format!("Failed to open {}.", path.display()))?;
    
    writeln!(
        &mut file, 
        ": 1\nname: {}\nversion: {}\nlicense: {}\nsummary: {}\n",
        config.package.name, 
        config.package.version,
        config.package.license,
        config.package.summary,
    ).with_context(|| format!("Failed to write {}.", path.display()))?;
    
    if let Some(keywords) = &config.package.keywords {
        write!(&mut file, "keywords:")
            .with_context(|| format!("Failed to write {}.", path.display()))?;
        
        for keyword in keywords {
            write!(&mut file, " {keyword}")
                .with_context(|| format!("Failed to write {}.", path.display()))?;
        }
        
        writeln!(&mut file, "")
            .with_context(|| format!("Failed to write {}.", path.display()))?;
    }
    
    if let Some(categories) = &config.package.keywords {
        write!(&mut file, "topics:")
            .with_context(|| format!("Failed to write {}.", path.display()))?;
        
        for topic in categories {
            write!(&mut file, " {topic},")
                .with_context(|| format!("Failed to write {}.", path.display()))?;
        }
        
        writeln!(&mut file, "")
            .with_context(|| format!("Failed to write {}.", path.display()))?;
    }
    
    if let Some(readme) = &config.package.readme {
        writeln!(&mut file, "description-file: {}", readme.display())
            .with_context(|| format!("Failed to write {}.", path.display()))?;
    }
    
    if let Some(homepage) = &config.package.homepage {
        writeln!(&mut file, "url: {homepage}")
            .with_context(|| format!("Failed to write {}.", path.display()))?;
    }
    
    if let Some(doc) = &config.package.documentation {
        writeln!(&mut file, "doc-url: {doc}")
            .with_context(|| format!("Failed to write {}.", path.display()))?;
    }
    
    if let Some(repo) = &config.package.repository {
        writeln!(&mut file, "src-url: {repo}")
            .with_context(|| format!("Failed to write {}.", path.display()))?;
    }
    
    if let Some(dependencies) = &config.dependencies {
        if let Some(deps) = &dependencies.deps {
            for (k, v) in deps {
                match &v.info {
                    DependencyInfo::RemoteDependency(dep) => {
                        writeln!(&mut file, "depends: {k} {}", dep.version)
                            .with_context(|| format!("Failed to write {}", path.display()))?;
                    },
                    DependencyInfo::LocalDependency(dep) => {
                        writeln!(&mut file, "depends: {k} {}", dep.version)
                            .with_context(|| format!("Failed to write {}", path.display()))?;
                    },
                    DependencyInfo::GitDependency(dep) => {
                        writeln!(&mut file, "depends: {k} {}", dep.version)
                            .with_context(|| format!("Failed to write {}", path.display()))?;
                    }
                }
            }
        }
    }
    
    if let Some(b_dependencies) = &config.build_dependencies {
        if let Some(deps) = &b_dependencies.deps {
            for (k, v) in deps {
                match &v.info {
                    DependencyInfo::RemoteDependency(dep) => {
                        writeln!(&mut file, "depends: * {k} {}", dep.version)
                            .with_context(|| format!("Failed to write {}", path.display()))?;
                    },
                    DependencyInfo::LocalDependency(dep) => {
                        writeln!(&mut file, "depends: * {k} {}", dep.version)
                            .with_context(|| format!("Failed to write {}", path.display()))?;
                    },
                    DependencyInfo::GitDependency(dep) => {
                        writeln!(&mut file, "depends: * {k} {}", dep.version)
                            .with_context(|| format!("Failed to write {}", path.display()))?;
                    }
                }
            }
        }
    }
    
    Ok(())
}