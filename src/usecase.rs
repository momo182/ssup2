pub mod program_init;
pub mod modes_of_operation;
pub mod env_parser;
pub mod network_host_utils;
pub mod parse_network;
use std::collections::HashMap;
use std::{env, vec};
use chrono::Utc;
use crate::entity::help_displayer::HelpDisplayer;
use crate::entity::supfile::networks::{HostDetails, HostEntry, Network, Networks};
use crate::entity::InitState;
use std::process;

#[allow(unused_imports)]
#[allow(dead_code)]
pub fn ensure_network_exists(network_name: &str, conf: &InitState, help_menu: &HelpDisplayer) {
    let networks = conf.supfile.networks.clone();
    if !networks.names.contains(&network_name.to_string()) {
        help_menu.show(conf);
        eprintln!("Error: Network '{}' does not exist", network_name);
        process::exit(1);
    }
}

#[allow(unused_imports)]
#[allow(dead_code)]
/// Overrides environment variables in the network with values from the given map.
pub fn override_env_from_args(env_from_args: &HashMap<String, String>, network: &mut Network) {
    match network.env {
        Some(ref mut env) => {
            for (key, value) in env_from_args {
                env.insert(key.clone(), value.clone());
            }
        },
        None => {
            let mut env = HashMap::new();
            for (key, value) in env_from_args {
                env.insert(key.clone(), value.clone());
            }
            network.env = Some(env);
        }
    }
}


#[allow(unused_imports)]
#[allow(dead_code)]
pub fn add_ssup_default_envs(network: &mut Network, init_state: &InitState) {
    let mut env = HashMap::<String, String>::new();
    if network.name == "localhost" {
        let network_key = "SUP_NETWORK".to_string();
        let network_value = "localhost".to_string();
        env.insert(network_key, network_value);
    } else {
        let network_key = "SUP_NETWORK".to_string();
        let network_value = init_state.args.to_vec()[0].clone();
        env.insert(network_key, network_value);
    }
    
    let now = Utc::now().to_rfc3339();
    let time_key = "SUP_TIME".to_string();
    let time_value = now;
    env.insert(time_key, time_value);

    if env.contains_key("SUP_USER") {
        let user_key = "SUP_USER".to_string();
        let user_value = env::var("SUP_USER").unwrap();
        env.insert(user_key, user_value);
    } else {
        let user_key = "SUP_USER".to_string();
        let user_value = env::var("USER").unwrap();
        env.insert(user_key, user_value);
    }

    // if network env does not exists set it to env
    if let None = network.env {
        network.env = Some(env);
    } else {
        // if network env exists, append to it
        let mut network_env = network.env.clone().unwrap();
        for (key, value) in env.iter() {
            network_env.insert(key.clone(), value.clone());
        }
        network.env = Some(network_env);
    }

}


#[allow(unused_imports)]
#[allow(dead_code)]
pub fn ensure_localhost(init_state: &mut InitState) {
    let mut got_local = false;
    let mut networks = init_state.supfile.networks.clone();
    if networks.is_empty() {
        got_local = false;
    }

    if let None = networks.get("localhost") {
        got_local = false;
    } else {
        got_local = true;
    }

    if !got_local {
        println!("adding localhost");
        if networks.is_empty() {
            let host_details = HostDetails::new("localhost".to_string());
            let host_entry = HostEntry::Detailed(host_details);
            let localhost_network = Network {
                hosts: vec![host_entry],
                env: None,
                inventory: None,
                bastion: None,
                user: None,
                pass: None,
                id_file: None,
                name: "localhost".to_string(),
            };
            let networks = Networks::new("localhost",&localhost_network);
            init_state.supfile.networks = networks;
        } else {
            let host_details = HostDetails::new("localhost".to_string());
            let host_entry = HostEntry::Detailed(host_details);
            let localhost_network = Network {
                hosts: vec![host_entry],
                env: None,
                inventory: None,
                bastion: None,
                user: None,
                pass: None,
                id_file: None,
                name: "localhost".to_string(),
            };

            if let Some(_) = networks.get("localhost") {
                // localhost network exists, skip adding it
                return;
            } else {
                // localhost network does not exist, add it
                networks.add_network("localhost".to_string(), localhost_network);
                init_state.supfile.networks = networks;
            }
        }

    }

}
