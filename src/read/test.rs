use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Test {
    pub command: String,
    pub args: Option<Vec<String>>,
    
    pub description: String,
    
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub stdin: Option<String>,
    
    pub exit: Option<String>
}