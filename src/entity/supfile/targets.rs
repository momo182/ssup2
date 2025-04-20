use serde::{Deserialize, Deserializer};
use serde_yaml::{self, Mapping, Value};
use std::collections::HashMap;
use std::fmt;

use super::networks::Network;

// #[derive(Debug, Deserialize, Clone)]
// #[derive(serde::Serialize)]
// pub struct AffixMapping {
//     pub target_name: String,
//     pub affixed_network: String,
//     pub command_name: String,
// }

#[derive(Debug, Deserialize, Clone)]
#[derive(serde::Serialize)]
pub struct Target {
    pub command: String,
    pub affixed_network: String,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize)]
pub struct Targets {
    pub names: Vec<String>,
    pub targets: HashMap<String, Vec<Target>>,
}

impl<'de> Deserialize<'de> for Targets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        // dbg!(value.clone());

        let targets_tpl: HashMap<String, Vec<String>> =
            serde_yaml::from_value(value.clone()).map_err(serde::de::Error::custom)?;

        let mut names = Vec::with_capacity(targets_tpl.len());
        let mut targets_final = HashMap::<String, Vec<Target>>::new();
        
        // targets
        for (key, lines) in targets_tpl.clone() {
            names.push(key.clone());
            let mut targets_generated: Vec<Target> = Vec::new();
            let mut command_name = String::new();
            // lines 
            for line in lines.iter() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let mut mapping: Target = Target {
                    command: "".to_string(),
                    affixed_network: "".to_string(),
                };

                if parts.len() == 2 {
                    command_name = parts[0].to_string();
                    let affixed_network = parts[1].to_string();

                    mapping = Target {
                        affixed_network: affixed_network.clone(),
                        command: command_name,
                    };

                } else if parts.len() == 1 {
                    command_name = parts[0].to_string(); // incorrect
                    mapping = Target {
                        affixed_network: "".to_string(),
                        command: command_name,
                    };
                } else {
                    println!("incorrect number of fields when splitting target mapping");
                    std::process::exit(2);
                }

                targets_generated.insert(0, mapping);
            } // dns lines
            targets_final.insert(key, targets_generated);
        } // end targets


        Ok(Targets {
            names,
            targets: targets_final,
        })
    }
}


impl Targets {
    pub fn get(&self, name: &str) -> Vec<Target> {
        let result = match self.targets.get(name) {
            None => {
                println!("no target found");
                std::process::exit(1);
            },
            Some(target) => target.clone(),
        };
        return result;
    }

    pub fn has(&self, name: &str) -> bool {
        self.targets.contains_key(name)
    }


    // pub fn has_affixes(&self) -> bool {
    //     !self.affixes.is_empty()
    // }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

impl fmt::Display for Targets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Targets {{ names: {:?}, targets: {:?} }}",
            self.names, self.targets,
        )
    }
}