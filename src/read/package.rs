use std::path::PathBuf;

use serde::{Deserialize};

use semver::Version;
use spdx_expression::SpdxExpression;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub summary: String,
    pub license: SpdxExpression,
    
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub readme: Option<PathBuf>,
    
    pub homepage: Option<Url>,
    pub documentation: Option<Url>,
    pub repository: Option<Url>,
}