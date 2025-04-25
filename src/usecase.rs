pub mod program_init;
pub mod modes_of_operation;
pub mod env_parser;
pub mod network_host_utils;
pub mod inventory_tools;
use std::collections::HashMap;
use std::{env, vec};
use chrono::Utc;
use crate::entity::help_displayer::HelpDisplayer;
use crate::entity::supfile::networks::{Host, HostEntry, Network, Networks};
use crate::entity::InitState;
use crate::gateways::logger::Logger;
use crate::usecase::inventory_tools::check_hosts_form;
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
pub fn override_env_from_args(env_from_args: &Option<HashMap<String, String>>, network: &mut Network) {
    let l = Logger::new("uc::override_env_from_args");
    if env_from_args.is_none() {
        l.log("no env values were supplied, will add nothing...");
        return;
    }

    let env_from_args = env_from_args.as_ref().unwrap();

    let mut env = network.env.clone();
    if !env.is_empty() {
        l.log(format!("overriding env variables with values from args: {:?}", &env_from_args));
        for (key, value) in env_from_args {
            env.insert(key.clone(), value.clone());
        }
    }

}


#[allow(unused_imports)]
#[allow(dead_code)]
pub fn add_ssup_default_envs(network: &mut Network, init_state: &InitState) {
    let mut env = HashMap::<String, String>::new();

    // set network name variable
    if network.name == "localhost" {
        let network_key = "SUP_NETWORK".to_string();
        let network_value = "localhost".to_string();
        env.insert(network_key, network_value);
    } else {
        let network_key = "SUP_NETWORK".to_string();
        let network_value = init_state.args.to_vec()[0].clone();
        env.insert(network_key, network_value);
    }
    
    // set time variable
    let now = Utc::now().to_rfc3339();
    let time_key = "SUP_TIME".to_string();
    let time_value = now;
    env.insert(time_key, time_value);

    // set user variable
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
    if network.env.is_empty() {
        network.env = env;
    } else {
        // if network env exists, append to it
        let mut network_env = network.env.clone();
        for (key, value) in env.iter() {
            network_env.insert(key.clone(), value.clone());
        }
        network.env = network_env;
    }

}


#[allow(unused_imports)]
#[allow(dead_code)]
pub fn ensure_localhost(init_state: &mut InitState) {
    let mut got_local = false;
    let mut networks = init_state.supfile.networks.clone();

    if let None = networks.get("localhost") {
        got_local = false;
    } else {
        got_local = true;
    }

    if !got_local {
        println!("adding localhost");
        if networks.is_empty() {
            let host_details = check_hosts_form("localhost");
            let host_env = HashMap::<String, String>::new();
            let localhost_network = Network {
                hosts: vec![host_details],
                env: host_env,
                inventory: "".to_string(),
                bastion: "".to_string(),
                user: "".to_string(),
                pass: "".to_string(),
                id_file: "".to_string(),
                name: "localhost".to_string(),
            };
            let networks = Networks::new("localhost",&localhost_network);
            init_state.supfile.networks = networks;
        } else {
            if let Some(_) = networks.get("localhost") {
                // localhost network exists, skip adding it
                return;
            } else {
                // localhost network does not exist, add it
                let host_details = check_hosts_form("localhost");
                let host_env = HashMap::<String, String>::new();
                let localhost_network = Network {
                    hosts: vec![host_details],
                    env: host_env,
                    inventory: "".to_string(),
                    bastion: "".to_string(),
                    user: "".to_string(),
                    pass: "".to_string(),
                    id_file: "".to_string(),
                    name: "localhost".to_string(),
                };
                networks.add_network("localhost".to_string(), localhost_network);
                init_state.supfile.networks = networks;
            }
        }

    }

}
