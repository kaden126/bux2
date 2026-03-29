use std::fmt::Display;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Toolchain {
    pub poptions: Option<Vec<String>>,
    pub coptions: Option<Vec<String>>,
    pub loptions: Option<Vec<String>>,
    
    pub compiler: Option<String>,
    pub std: Option<String>,
    
    pub modules: Option<bool>
}

impl Display for Toolchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(compiler) = &self.compiler {
            writeln!(f, "config.cxx={compiler}")?;
        }
        
        if let Some(std) = &self.std {
            writeln!(f, "config.cxx.std={std}")?;
        }
        
        if let Some(modules) = &self.modules {
            writeln!(f, "config.cxx.features.modules={modules}")?;
        }
        
        if let Some(coptions) = &self.coptions {
            for option in coptions {
               writeln!(f, "config.cxx.coptions+={option}")?;
            }
        }
        
        if let Some(poptions) = &self.poptions {
            for option in poptions {
                writeln!(f, "config.cxx.poptions+={option}")?;
            }
        }
        
        if let Some(loptions) = &self.loptions {
            for option in loptions {
                 writeln!(f, "config.cxx.loptions+={option}")?;
            }
        }
        
        Ok(())
    }
}