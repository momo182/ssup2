use crate::entity::supfile;
use crate::entity::help_displayer::HelpDisplayer;
use crate::entity::supfile::networks::{Host, Network};
use crate::entity::CommandLineArgs;
use crate::usecase::ssh_config_parser::{must_parse_ssh_config, SSHHost};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use crate::entity::{InitState, playbook::PlayBook};
use crate::usecase::modes_of_operation::{special_target_mode::special_target_mode, normal_mode::normal_mode, makefile_mode::makefile_mode};
use crate::gateways::logger::Logger;
use regex::Regex;

use super::inventory_tools::resolve_path;


pub fn parse_supfile(args: CommandLineArgs) -> supfile::Supfile {
    let l = Logger::new("uc::program_init::parse_supfile");
    let mut file_to_read: std::path::PathBuf;
    let start_dir = env::current_dir().expect("failed to get current dir");
    l.log(&format!("supfile parsing started in folder: {:?}", start_dir));

    if args.file == "" {
        file_to_read = std::path::PathBuf::from(".".to_string());
        file_to_read.push("Supfile.yml");
    } else {
        file_to_read = std::path::PathBuf::from(args.file.as_str());
    }

    // println!("file_to_read: {}", file_to_read.display());
    
    let content = std::fs::read_to_string(file_to_read.clone());
    if let Err(error) = content {
        // // exit to os with non-zero status
        println!("Could not open file: {}", file_to_read.display());
        println!("reason: {}", error);
        println!("Using default Supfile");
        file_to_read = std::path::PathBuf::from(".".to_string());
        file_to_read.push("Supfile");
    }
    
    let content = std::fs::read_to_string(file_to_read.clone());
    if let Err(error) = content {
        // exit to os with non-zero status
        println!("Could not open file: {}", file_to_read.display());
        println!("reason: {}", error);
        std::process::exit(1);
    }
    let content = content.unwrap();
    let full_path = get_full_path(file_to_read.clone());
    let full_path = std::path::PathBuf::from(full_path);
    let basedir = full_path.parent().expect("directory probably doesnt exist");
    
    // println!("start_dir is {:?}", start_dir);
    // println!("basedir is {:?}", basedir);
    // println!("content: {}", content);

    if !content.is_empty() {
        // println!("cd to supfile dir: {}", basedir.to_str().unwrap());

        if let Err(e) = env::set_current_dir(basedir) {
            eprintln!("failed to cd to new Wd: {}", e);
            std::process::exit(1);
        }

        // println!("cd done");
    }

    l.log("done parsing supfile");
    return serde_yaml::from_str(&content).expect(format!("could not parse file: {}", file_to_read.display()).as_str());

}


fn get_full_path(relative_path: PathBuf) -> String {
    match fs::canonicalize(relative_path) {
        Ok(path) => path.to_string_lossy().into_owned(),
        Err(_) => String::from("Error resolving path"),
    }
}

fn usage_on_no_args(init_data: InitState) {
    let mut help_menu = HelpDisplayer::new(init_data.clone());
    if init_data.args.len() < 1 {
        help_menu.show_all(&init_data);
        let message = "Usage: ssup [OPTIONS] NETWORK COMMAND [...]\n       ssup [ --help | -v | --version ]";
        println!("{}",message);
        std::process::exit(2)
    }
}


pub fn parse_initial_args(init_data: &mut InitState) -> PlayBook {
    let l = Logger::new("uc::program_init::parse_initial_args");
    let conf = init_data.supfile.clone();
    let args = &init_data.args;
    let args_count = args.len();
    let mut help_menu = HelpDisplayer::new(init_data.clone());

    l.log(format!("Checking if we have any args at all, len: {}", args_count).as_str());

    if args_count == 0 {
        usage_on_no_args(init_data.clone());
    }

    if !conf.networks.is_empty() {
        if all_args_are_targets(&init_data) {
            l.log("Special target mode");
            return special_target_mode(&init_data, &mut help_menu);
        }
        
        l.log("Normal mode");
        return normal_mode(&init_data, &help_menu);
    }

    l.log("Makefile mode");
    makefile_mode(init_data, &mut help_menu)
}

fn all_args_are_targets(init_data: &InitState) -> bool {
    let l = Logger::new("uc::program_init::all_args_are_targets");
    let conf = &init_data.supfile;
    let args = &init_data.args;
    
    l.log(format!("Checking if all given args are targets: {}", args.len()));
    
    let targets = conf.targets.clone();

    for single_argument in args {
        l.log(format!("Targets check -> checking {}", single_argument).as_str());
        if !targets.has(single_argument) {
            l.log(format!("Targets check -> unknown keyword: {}", single_argument));
            return false;
        } else {
            l.log(format!("Targets check -> keyword found: {}", single_argument));
        }
    }
    
    true
}

// come from CheckInitialArgs
pub fn check_additional_flags(selected_network: &mut Network,init_data: &mut InitState) {
    let l = Logger::new("uc::program_init::check_additional_flags");

    l.log("--only flag filters hosts");
    if !init_data.flags.onlyhosts.is_empty() {
        process_only_flags(selected_network,init_data);
    }

    l.log("--exclude flag filters hosts");
    if !init_data.flags.excepthosts.is_empty() {
        process_exclude_flags(selected_network,init_data);
    }

    l.log("--sshconfig flag location for ssh_config file");
    if !init_data.flags.sshconfig.is_empty() {
        process_sshconfig_flag(selected_network,init_data);
    }
}



pub fn process_sshconfig_flag(selected_network: &mut Network,init_data: &mut InitState) {
    let l = Logger::new("uc::program_init::process_sshconfig_flag");
    let mut config_map: HashMap<String, SSHHost> = HashMap::new();
    
    // fisrts lets read the config file
    // and grab all the hosts defined there
    l.log(format!("reading sshconfig: {}", init_data.flags.sshconfig).as_str());
    let ssh_config_full_path = resolve_path(&init_data.flags.sshconfig.clone());
    let hosts_from_config = must_parse_ssh_config(ssh_config_full_path.clone().as_str());

    // as one ssh config may be assigned to multiple hosts,
    // we need to iterate through all of them
    // and explicitly add them to our hostmap
    l.log("forming conf map");
    for ssh_host in hosts_from_config {
        for single_host in ssh_host.host.iter() {
            config_map.insert(single_host.clone(), ssh_host.clone());
        }
    }

    l.log("range over hosts and check config map for host present");
    for single_host in &mut selected_network.hosts {
        if config_map.contains_key(single_host.host.as_str()) {
            l.log(format!("{} -> adding ssh config", single_host.host).as_str());

            let ssh_config = config_map.get(single_host.host.as_str()).unwrap().clone();
            let id_file_relative_path = ssh_config.identity_file.expect("973D413B-52B7-4E9B-8260-E537E52C7758: identity file is supposed to exist");
            let id_file_full_path = get_full_path(PathBuf::from(id_file_relative_path));

            selected_network.user = ssh_config.user.expect("DED9325C-B06F-4186-84D8-8804F0C0F1E0: user is supposed to exist");
            selected_network.id_file = id_file_full_path;
            let host_with_port = format!("{}:{}", single_host.host, ssh_config.port);
            single_host.host = host_with_port;
        }
    }
}



pub fn process_only_flags(selected_network: &mut Network, init_data: &mut InitState) {
    let l = Logger::new("uc::program_init::process_only_flags");
    l.log("Processing --only flag");

    // Compile the regex for the --only flag
    let only_hosts_pattern = &init_data.flags.onlyhosts;
    let expr = match Regex::new(only_hosts_pattern) {
        Ok(regex) => regex,
        Err(e) => {
            eprintln!("Invalid regex for --only flag: {}", e);
            std::process::exit(44);
        }
    };

    // Filter hosts that match the regex
    let mut filtered_hosts: Vec<Host> = Vec::new();
    for host in &selected_network.hosts {
        if expr.is_match(&host.host) {
            filtered_hosts.push(host.clone());
        }
    }

    // Log the number of hosts found
    l.log(format!("Found 'only' hosts: {}", filtered_hosts.len()).as_str());

    // Exit if no hosts match the --only flag
    if filtered_hosts.is_empty() {
        eprintln!(
            "No hosts match --only '{}'",
            init_data.flags.onlyhosts
        );
        std::process::exit(45);
    }

    // Update the network's hosts with the filtered list
    selected_network.hosts = filtered_hosts;
}


pub fn process_exclude_flags(selected_network: &mut Network, init_data: &mut InitState) {
    let l = Logger::new("uc::program_init::process_exclude_flags");
    l.log("Processing --exclude flag");

    // Compile the regex for the --exclude flag
    let except_hosts_pattern = &init_data.flags.excepthosts;
    let expr = match Regex::new(except_hosts_pattern) {
        Ok(regex) => regex,
        Err(e) => {
            eprintln!("Invalid regex for --exclude flag: {}", e);
            std::process::exit(42);
        }
    };

    // Filter out hosts that match the regex
    let mut filtered_hosts = Vec::new();
    for host in &selected_network.hosts {
        if !expr.is_match(&host.host) {
            filtered_hosts.push(host.clone());
        }
    }

    // Log the number of hosts left after exclusion
    l.log(format!("Found 'except' hosts: {}", filtered_hosts.len()).as_str());

    // Exit if no hosts are left after applying the --exclude flag
    if filtered_hosts.is_empty() {
        eprintln!(
            "No hosts left after --except '{}'",
            init_data.flags.excepthosts
        );
        std::process::exit(43);
    }

    // Update the network's hosts with the filtered list
    selected_network.hosts = filtered_hosts;
}