use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub name: String,
    
    #[serde(rename = "type")]
    pub kind: TargetType,

    pub src: PathBuf,
    
    pub testing: Option<bool>
}

#[derive(Debug, Deserialize)]
pub enum TargetType {
    #[serde(
        alias = "lib",
        alias = "Lib",
        alias = "LIB"
    )]
    Lib,
    
    #[serde(
        alias = "exe",
        alias = "Exe",
        alias = "EXE"
    )]
    Exe
}