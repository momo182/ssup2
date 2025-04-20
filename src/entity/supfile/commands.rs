use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use crate::entity::supfile::UploadEntry;
use crate::entity::supfile::FetchEntry;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub desc: Option<String>,
    pub run: Option<String>,
    pub upload: Option<Vec<UploadEntry>>, 
    pub fetch: Option<FetchEntry>,
    pub env: Option<HashMap<String, String>>, 
    pub local: Option<String>,
    pub stdin: Option<bool>,
    #[serde(skip)]
    pub name: String,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let empty = "no descr".to_string();
        let description = self.desc.as_ref().unwrap_or(&empty).trim();
        write!(f, "{}", description)
    }
}
