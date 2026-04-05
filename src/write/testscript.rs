use anyhow::Context;

use crate::read::Config;
use std::io::Write;

pub fn write_testscripts(config: &Config) -> anyhow::Result<()> {
    let path = std::env::current_dir().with_context(|| "Failed to retrieve working directory.")?;

    if let Some(tests) = &config.test {
        for test in tests {
            let testscript_path = path.join(&test.src).join("testscript");

            let mut file = std::fs::File::create(&testscript_path)
                .with_context(|| format!("Failed to open {}", testscript_path.display()))?;

            writeln!(&mut file, "{test}")
                .with_context(|| format!("Failed to write to {}", testscript_path.display()))?;
        }
    }

    Ok(())
}
