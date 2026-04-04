use anyhow::Context;
use bux2::cli::Commands;
use bux2::cli::process;

fn main() -> anyhow::Result<()> {
    let cli = bux2::parse_cli();

    match cli.command {
        Commands::Build { profile } => {
            process::build(&profile).with_context(|| "Failed to run build process.")?;
        }
        Commands::Generate => {
            bux2::generate_build_files().with_context(|| "Failed to generate build files.")?;
        }
        Commands::Clean { profile, recursive } => {
            process::clean(&profile, recursive)
                .with_context(|| "Failed to clean build artifacts.")?;
        }
        Commands::Ci => {
            process::ci().with_context(|| "Failed run ci.")?;
        }
        Commands::DeinitProfile { profile } => {
            process::deinit(&profile).with_context(|| "Failed deinitialize profile.")?;
        }
        Commands::Fetch { profile } => {
            process::fetch(&profile).with_context(|| "Failed to fetch.")?;
        }
        Commands::InitProfile { profile } => {
            process::init(&profile).with_context(|| "Failed to initialize profile.")?;
        }
        Commands::Publish => {
            process::publish().with_context(|| "Failed to publish.")?;
        }
        Commands::New { name, template } => {
            process::new(&name, template).with_context(|| "Failed to generate new project.")?;
        }
    }

    Ok(())
}
