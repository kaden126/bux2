use std::fmt::Display;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    pub sources: Option<String>,
    pub headers: Option<String>,
    pub modules: Option<String>,
}

impl Display for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f, 
            "hxx{{*}}: extension = {}\ncxx{{*}}: extension = {}\nmxx{{*}}: extension = {}",
            self.headers.as_ref().unwrap_or(&"hxx".into()),
            self.sources.as_ref().unwrap_or(&"cxx".into()),
            self.modules.as_ref().unwrap_or(&"mxx".into())
        )
    }
}

#[cfg(test)]
mod tests {
    use toml::toml;
    use super::*;

    #[test]
    fn parse_extensions() {
        let toml = toml! {
            sources = "cpp"
            header = "hpp"
            modules = "mpp"
        };
        Extensions::deserialize(toml).unwrap();
    }
}