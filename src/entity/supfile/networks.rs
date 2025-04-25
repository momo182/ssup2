use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{hash_map, HashMap};
use std::fmt::{self};
use crate::usecase::inventory_tools::check_hosts_form;



#[derive(Debug,Serialize, Clone)]
pub struct Networks {
    pub names: Vec<String>,
    pub nets: HashMap<String, Network>
}

#[derive(Debug, Serialize, Clone)]
pub struct Network {
    pub hosts: Vec<Host>,
    pub env: HashMap<String, String>,
    pub inventory: String,
    pub bastion: String,
    pub user: String,
    pub pass: String,
    pub id_file: String,
    #[serde(skip)]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkOptionalMode {
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
    Detailed(Host),
}



// type NetworkHost struct {
// 	Host     string  `yaml:"host"`
// 	User     string  `yaml:"user"`
// 	Password string  `yaml:"pass"`
// 	Tube     string  `yaml:"tube"`
// 	Env      EnvList `yaml:"env"`
// 	Sudo     bool    `yaml:"sudo" default:"false"`
// }



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Host {
    pub host: String,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub tube: Option<String>,
    pub env: Option<HashMap<String, String>>,
    #[serde(default)]
    pub sudo: bool,
}

impl Host {
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fn new(host: String) -> Host {
        Host { host, user: None, pass: None, tube: None, env: None, sudo: false }
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

impl fmt::Display for Host {
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
        let map: HashMap<String, NetworkOptionalMode> = Deserialize::deserialize(deserializer)?;
        let mut nets: HashMap<String, Network> = HashMap::new();
        let mut names = Vec::new();

        for (name, network) in map {
            let mut network_to_add = Network{
                hosts: Vec::new(),
                env: hash_map::HashMap::new(),
                inventory: "".to_string(),
                bastion: "".to_string(),
                user: "".to_string(),
                pass: "".to_string(),
                id_file: "".to_string(),
                name: "".to_string(),
            };

            for host in network.hosts {
                let mut host_to_add = Host{
                    host: "".to_string(),
                    user: None,
                    pass: None,
                    tube: None,
                    env: None,
                    sudo: false,
                };
                match host {
                    HostEntry::Simple(simple_host) => {
                        let processed_host = check_hosts_form(&simple_host);
                        host_to_add.host = processed_host.host;
                        host_to_add.pass = processed_host.pass;
                        host_to_add.tube = processed_host.tube;
                        host_to_add.sudo = processed_host.sudo;
                        host_to_add.user = processed_host.user;
                    },
                    HostEntry::Detailed(details) => {
                        host_to_add.host = details.host;
                        host_to_add.user = details.user.clone();
                        host_to_add.pass = details.pass.clone();
                        host_to_add.tube = details.tube.clone();
                        host_to_add.env = details.env.clone();
                        host_to_add.sudo = details.sudo;
                }
            }

            network_to_add.name = name.clone();
            if network.env.is_some() {
                network_to_add.env = network.env.clone().expect("No env");
            }
            if network.inventory.is_some() {
                network_to_add.inventory = network.inventory.clone().expect("No inventory");
            }
            if network.bastion.is_some() {
                network_to_add.bastion = network.bastion.clone().expect("No bastion");
            }
            if network.user.is_some() {
                network_to_add.user = network.user.clone().expect("No user");
            }
            if network.pass.is_some() {
                network_to_add.pass = network.pass.clone().expect("No pass");
            }
            if network.id_file.is_some() {
                network_to_add.id_file = network.id_file.clone().expect("No id_file");
            }
            network_to_add.hosts.push(host_to_add);

            // network_to_add.inventory = network.inventory.clone().expect("No inventory");
            // network_to_add.bastion = network.bastion.clone().expect("No bastion");
            // network_to_add.user = network.user.clone().expect("No user");
            // network_to_add.pass = network.pass.clone().expect("No pass");
            // network_to_add.id_file = network.id_file.clone().expect("No id_file");
            nets.insert(name.clone(), network_to_add.clone());
            names.push(name.clone());
            }
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
            env: hash_map::HashMap::new(),
            inventory: "".to_string(),
            bastion: "".to_string(),
            user: "".to_string(),
            pass: "".to_string(),
            id_file: "".to_string(),
            name: name.to_string(),
        };
        let mut networks = Networks {
            nets: HashMap::new(),
            names: Vec::new(),
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




