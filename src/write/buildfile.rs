use crate::read::{Config, target::TargetType};
use anyhow::Context;
use std::io::Write;

pub fn write_top_buildfile() -> anyhow::Result<()> {
    let path = std::env::current_dir()
        .with_context(|| "Failed to retrieve working directory.")?
        .join("buildfile");

    let mut file = std::fs::File::create(&path)
        .with_context(|| format!("Failed to open {}.", path.display()))?;

    writeln!(&mut file, "./: {{*/ -build/}}")
        .with_context(|| format!("Failed to write {}.", path.display()))?;

    Ok(())
}

pub fn write_buildfiles(config: &Config) -> anyhow::Result<()> {
    for target in &config.target {
        let path = std::env::current_dir()
            .with_context(|| "Failed to retrieve working directory.")?
            .join(&target.src)
            .join("buildfile");

        let mut file = std::fs::File::create(&path)
            .with_context(|| format!("Failed to open {}.", path.display()))?;

        writeln!(&mut file, "libs =")
            .with_context(|| format!("Failed to write {}", path.display()))?;

        if let Some(dependencies) = &config.dependencies {
            if let Some(deps) = &dependencies.deps {
                for (k, v) in deps {
                    for target in &v.targets {
                        writeln!(&mut file, "import libs += {}%lib{{{}}}", k, target)
                            .with_context(|| format!("Failed to write {}.", path.display()))?;
                    }
                }
            }
        }

        match target.kind {
            TargetType::Exe => {
                if let Some(true) = target.testing {
                    writeln!(
                        &mut file,
                        "exe{{{}}}: {{hxx cxx mxx}}{{**}} $libs testscript",
                        target.name
                    )
                    .with_context(|| format!("Failed to write {}.", path.display()))?;
                } else {
                    writeln!(
                        &mut file,
                        "exe{{{}}}: {{hxx cxx mxx}}{{**}} $libs",
                        target.name
                    )
                    .with_context(|| format!("Failed to write {}.", path.display()))?;
                }
            }

            TargetType::Lib => {
                writeln!(
                    &mut file,
                    "lib{{{}}}: {{hxx cxx mxx}}{{**}} $libs",
                    target.name
                )
                .with_context(|| format!("Failed to write {}.", path.display()))?;
            }
        }
    }

    Ok(())
}
