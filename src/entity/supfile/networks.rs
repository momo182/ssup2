use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt::{self};



#[derive(Debug,Serialize, Clone)]
pub struct Networks {
    pub names: Vec<String>,
    pub nets: HashMap<String, Network>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    pub hosts: Vec<HostEntry>,
    pub env: Option<HashMap<String, String>>,
    pub inventory: Option<String>,
    pub bastion: Option<String>,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub id_file: Option<String>,
    #[serde(skip)]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum HostEntry {
    Simple(String),
    Detailed(HostDetails),
}



// type NetworkHost struct {
// 	Host     string  `yaml:"host"`
// 	User     string  `yaml:"user"`
// 	Password string  `yaml:"pass"`
// 	Tube     string  `yaml:"tube"`
// 	Env      EnvList `yaml:"env"`
// 	Sudo     bool    `yaml:"sudo" default:"false"`
// 	// Namespace string  `yaml:"namespace" default:""`
// }



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HostDetails {
    pub host: String,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub tube: Option<String>,
    pub env: Option<HashMap<String, String>>,
    #[serde(default)]
    pub sudo: bool,
}

impl HostDetails {
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fn new(host: String) -> HostDetails {
        HostDetails { host, user: None, pass: None, tube: None, env: None, sudo: false }
    }
}

impl fmt::Display for HostEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HostEntry::Simple(s) => {
                // split on " | "
                let parts = s.split(" | ").collect::<Vec<&str>>();
                let header = parts.get(0).unwrap();
                // and print out the header, dropping sensitive information
                write!(f, "{}", header)
            },
            HostEntry::Detailed(details) => write!(f, "{}", details),
        }
    }
}

impl fmt::Display for HostDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HostDetails {{ host: {:?}, pass: {:?}, tube: {:?}, env: {:?} }}", 
            self.host, self.pass, self.tube, self.env)
    }
}

impl<'de> Deserialize<'de> for Networks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {   
        let map: HashMap<String, Network> = Deserialize::deserialize(deserializer)?;
        let mut nets: HashMap<String, Network> = HashMap::new();
        let mut names = Vec::new();

        for (name, mut network) in map {
            network.name = name.clone();
            nets.insert(name.clone(), network);
            names.push(name);
        }

        Ok(Networks{ names, nets })
    }
}

impl Networks {
    pub fn get(&self, name: &str) -> Option<&Network> {
        self.nets.get(name)
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fn new( name: &str, network: &Network) -> Networks {
        let hosts = network.hosts.clone();
        let network = Network {
            hosts: hosts,
            env: None,
            inventory: None,
            bastion: None,
            user: None,
            pass: None,
            id_file: None,
            name: name.to_string(),
        };
        let mut networks = Networks {
            names: Vec::new(),
            nets: HashMap::new(),
        };
        networks.add_network(name.to_string(), network);
        networks
    }

    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Network> {
        self.nets.get_mut(name)
    }

    pub fn add_network(&mut self, name: String, network: Network) {
        self.names.push(name.clone());
        self.nets.insert(name, network);
    }

}



