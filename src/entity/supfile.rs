pub mod targets;
pub mod networks;
pub mod commands;
use crate::entity::supfile::commands::Command;
use crate::entity::supfile::targets::Targets;
use networks::Networks;
use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;
use indexmap::IndexMap;

#[derive(Debug, Serialize, Clone)]
pub struct Supfile {
    pub desc: String,
    pub version: String,
    pub networks: Networks,
    pub env: HashMap<String, String>, 
    pub commands: HashMap<String, Command>,
    pub targets: Targets,
}

impl<'de> Deserialize<'de> for Supfile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First deserialize into a temporary struct that matches the YAML exactly
        #[derive(Deserialize, Debug, Clone)]
        struct TempSupfile {
            pub version: String,
            pub env: Option<HashMap<String, String>>,
            pub networks: Option<Networks>,
            pub commands: HashMap<String, Command>,
            pub targets: Option<Targets>,
            pub desc: Option<String>,
        }

        let temp = TempSupfile::deserialize(deserializer)?;
        // dbg!(temp.clone());

        // Now create the proper Supfile with command names set
        let mut commands = HashMap::new();
        for (name, mut command) in temp.commands {
            command.name = name.clone();
            commands.insert(name, command);
        }

        let null_networks = Networks {
            names: vec![],
            nets: HashMap::new(),
        };

        let null_targets = Targets {
            names: vec![],
            targets: IndexMap::new(),
        };

        Ok(Supfile {
            version: temp.version,
            env: temp.env.unwrap_or(HashMap::new()),
            networks: temp.networks.unwrap_or(null_networks),
            commands,
            targets: temp.targets.unwrap_or(null_targets),
            desc: temp.desc.unwrap_or("no description".to_string()), 
        })
    }
}


impl Supfile {
    pub fn get_command(&self, name: &str) -> Option<&Command>{
        self.commands.get(name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadEntry {
    pub src: String,
    pub dst: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FetchEntry {
    pub src: String,
    pub dst: String,
}

