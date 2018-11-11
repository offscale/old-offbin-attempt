use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: Option<String>,
    pub depends: Vec<Cmd>,
    pub env:  Option<HashMap<String, String>>,
    pub pipe: Option<Cmd>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cmd {
    pub exec: String,
    pub args: Option<Vec<String>>,
    pub env:  Option<HashMap<String, String>>,
}