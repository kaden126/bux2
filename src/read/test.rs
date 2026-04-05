use serde::Deserialize;
use std::{fmt::Display, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Test {
    pub description: String,
    pub src: PathBuf,
    pub args: Option<Vec<String>>,

    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub stdin: Option<String>,

    pub exit: Option<String>,
}

impl Display for Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, ": {}", self.description)?;

        write!(f, "$*")?;

        if let Some(_) = &self.stdin {
            write!(f, " - <<__END_STDIN__")?;
        };

        if let Some(_) = &self.stdout {
            write!(f, " 1>>__END_STDOUT__")?;
        }

        if let Some(_) = &self.stderr {
            write!(f, " 2>>__END_STDERR__")?;
        }

        if let Some(exit) = &self.exit {
            write!(f, " {exit}")?;
        }

        if let Some(stdin) = &self.stdin {
            write!(f, "\n{stdin}\n__END_STDIN__")?;
        };

        if let Some(stdout) = &self.stdout {
            write!(f, "\n{stdout}\n__END_STDOUT__")?;
        }

        if let Some(stderr) = &self.stderr {
            write!(f, "\n{stderr}\n__END_STDERR__")?;
        }

        writeln!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::toml;

    #[test]
    fn parse_test() {
        let toml = toml! {
            description = "Test an important thing"
            src = "src"
            args = ["Hello", "World!"]

            stdout = "Hello"
            stderr = "World"
            stdin = "Input!"

            exit = "== 0"
        };
        Test::deserialize(toml).unwrap();
    }
}
