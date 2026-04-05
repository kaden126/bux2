use std::path::PathBuf;

use serde::Deserialize;

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

#[cfg(test)]
mod tests {
    use super::*;
    use toml::toml;

    #[test]
    fn parse_package() {
        let toml = toml! {
            name = "my-project"
            version = "0.1.0"
            summary = "testing for bux2"
            license = "MIT"

            keywords = ["testing", "validation"]
            categories = ["test the stuff!", "high quality testing"]
            readme = "README.md"

            homepage = "https://my-project-page.com"
            documentation = "https://my-project-docs.com"
            repository = "https://my-project-source.com"
        };
        Package::deserialize(toml).unwrap();
    }
}
