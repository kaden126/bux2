pub mod process;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about, author)]
/**
bux2: A refreshingly simple *B*uild2 *UX*.
 */
pub struct CliArgs {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "generate")]
    /**
    Only generate build2 files. Done automatically when running most other commands.
     */
    Generate,
    
    #[command(name = "init")]
    /**
    Initialize a profile specified in `bux2.toml`.
     */
    InitProfile {
        #[clap(short, long)]
        profile: String
    },
    
    /**
    Deinitialize and remove an active profile.
     */
    #[command(name = "deinit")]
    DeinitProfile {
        #[clap(short, long)]
        profile: String,
    },
    
    #[command(name = "fetch")]
    /**
    Fetch dependencies.
     */
    Fetch {
        #[clap(long, short)]
        profile: String
    },
    
    #[command(name = "clean")]
    /**
    Remove build artifacts.
     */
    Clean {
        #[clap(long, short)]
        profile: String,
        
        #[clap(long, short)]
        recursive: bool,
    },
    
    #[command(name = "build")]
    /**
    Build a profile.
     */
    Build {
        #[clap(long, short)]
        profile: String
    },
    
    #[command(name = "publish")]
    /**
    Publish the package to https://cppget.org.
     */
    Publish {
        #[clap(long, short)]
        profile: String
    },
    
    #[command(name = "ci")]
    /**
    Submit the package to build2 ci.
     */
    Ci {
        #[clap(long, short)]
        profile: String
    },
    
    #[command(name = "new")]
    /**
    Generate a new template package.
     */
    New {
        name: String
    }
}