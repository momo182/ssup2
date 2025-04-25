use serde::{Deserialize, Serialize, Deserializer};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fmt;
use crate::entity::supfile::UploadEntry;
use crate::entity::supfile::FetchEntry;


#[derive(Debug, Serialize, Clone)]
pub struct Command {
    pub desc: String,
    pub run: String,
    pub upload: Vec<UploadEntry>, 
    pub fetch: FetchEntry,
    pub env: HashMap<String, String>, 
    pub local: String,
    pub stdin: bool,
    #[serde(skip)]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CommandOptionalMode {
    pub desc: Option<String>,
    pub run: Option<String>,
    pub upload: Option<Vec<UploadEntry>>, 
    pub fetch: Option<FetchEntry>,
    pub env: Option<HashMap<String, String>>, 
    pub local: Option<String>,
    pub stdin: Option<bool>,
    #[serde(skip)]
    pub name: Option<String>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description: String = self.desc.trim().to_string();
        write!(f, "{}", description)
    }
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // initial deserialize dance
        let raw: Value = Deserialize::deserialize(deserializer)?;
        let command_template: CommandOptionalMode = serde_yaml::from_value(raw.clone()).map_err(serde::de::Error::custom)?;

        // prepare new command to be filled
        let mut command_final = Command {
            desc: String::from("no description"),
            run: String::from(""),
            upload: vec![],
            fetch: FetchEntry{
                src: String::from(""),
                dst: String::from(""),
            },
            env: HashMap::new(),
            local: String::from(""),
            stdin: false,
            name: String::from(""),
        };

        // fill the new command
        if let Some(desc) = command_template.desc {
            command_final.desc = desc;
        }

        if let Some(run) = command_template.run {
            command_final.run = run;
        }

        if let Some(upload) = command_template.upload {
            command_final.upload = upload;
        }

        if let Some(fetch) = command_template.fetch {
            command_final.fetch = fetch;
        }

        if let Some(env) = command_template.env {
            command_final.env = env;
        }

        if let Some(local) = command_template.local {
            command_final.local = local;
        }

        if let Some(stdin) = command_template.stdin {
            command_final.stdin = stdin;
        }

        if let Some(name) = command_template.name {
            command_final.name = name;
        }

        // dbg!(&command_final);

        Ok(command_final)   

    }
}


