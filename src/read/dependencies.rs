use std::{collections::HashMap, path::PathBuf};
use semver::VersionReq;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Dependencies {
   pub channels: Option<Vec<Channel>>,
   
   #[serde(flatten)]
   pub deps: Option<HashMap<String, Dependency>>
}

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub targets: Vec<String>,
    
    #[serde(flatten)]
    pub info: DependencyInfo
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DependencyInfo {
    LocalDependency(LocalDep),
    GitDependency(GitDep),
    RemoteDependency(RemoteDep),
}

#[derive(Debug, Deserialize)]
pub struct RemoteDep {
    pub version: VersionReq
}

#[derive(Debug, Deserialize)]
pub struct LocalDep {
    pub path: PathBuf,
    pub version: VersionReq
}

#[derive(Debug, Deserialize)]
pub struct GitDep {
    pub repo: Url,
    pub version: VersionReq
}

#[derive(Debug, Deserialize)]
pub enum Channel {
    #[serde(alias = "legacy")]
    Legacy,
    #[serde(alias = "stable")]
    Stable,
    #[serde(alias = "testing")]
    Testing,
    #[serde(alias = "beta")]
    Beta,
    #[serde(alias = "alpha")]
    Alpha,
}

#[cfg(test)]
mod tests {
    use toml::toml;
    use super::*;

    #[test]
    fn parse_dependencies() {
        let toml = toml! {
            channels = ["stable", "testing", "legacy"]
            libremote = { version = "1.0.0", targets = ["remote"] }
            libpath = { version = "1.0.0", path = "/Users/me/libgoodbye", targets = ["path"] }
            libgit = { version = "1.0.0", repo = "https://me/libgit.git", targets = ["git"] }
        };
        Dependencies::deserialize(toml).unwrap();
    }
}