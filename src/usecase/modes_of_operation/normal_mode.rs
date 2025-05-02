use crate::entity::help_displayer::HelpDisplayer;
use crate::entity::InitState;
use std::process;
use crate::gateways::logger::Logger;
use crate::entity::playbook::{PlayBook,Play};
// use crate::usecase::env_parser::parse_env; //TODO remove me
use crate::usecase::{ensure_network_exists,override_env_from_args,add_ssup_default_envs};
use crate::usecase::inventory_tools::parse_inventory;

#[allow(dead_code)]
/// Parses the initial arguments and constructs a playbook.
///
/// This function processes command-line arguments, retrieves network information,
/// parses inventory, and builds a playbook containing commands and targets.
///
/// # Errors
///
/// Returns an `AppInitError` if the network or command is not found, or if an invalid
/// command or target is specified.
pub fn normal_mode(init_data: &InitState, help_menu: &HelpDisplayer) -> PlayBook {
    let l = Logger::new("uc::modes_of_operation::normal_mode");
    
    l.log("create empty playbook");
    let mut result = PlayBook { 
        plays: Vec::new(),
        is_makefile: false,
    };

    l.log("parse CLI --env flag");
    // let env_from_args = parse_env(&init_data.flags.env);
    let env_from_args = init_data.flags.parse_env();
    l.log(format!("env from args: {:?}", env_from_args));
    let mut args = init_data.args.clone();

    let network_name = args.remove(0);

    l.log(format!("check if network is defined: {}", &network_name));
    ensure_network_exists(&network_name, &init_data, &help_menu);

    let mut network = init_data.supfile
        .networks
        .clone()
        .nets
        .get(&network_name.clone())
        .cloned()
        .expect(format!("Network '{}' not found", network_name).as_str());

    l.log("parse CLI --env flag env vars, override values defined in Network env");
    override_env_from_args(&env_from_args, &mut network);

    l.log("check if we have an inventory via script execution");
    // TODO: pase inventory should apply found hosts to network directly
    let hosts = parse_inventory(&network).expect("failed to parse inventory from script");
    network.hosts.extend(hosts);

    add_ssup_default_envs(&mut network, &init_data);

    let mut play = Play {
        network: network,
        commands: Vec::new(),
    };


    for single_argument in args {
        l.log(&format!("find if given arg is command or target: {}", single_argument));
        
        let commands = init_data.supfile.commands.clone();
        let targets = init_data.supfile.targets.clone();
        let mut is_command_found = false;
        dbg!(&commands);
        dbg!(&targets);
        
        l.log("check if its a command");
        if let Some(command) = commands.get(&single_argument) {
            l.log(&format!("found command: {}", single_argument));
            play.commands.push(command.clone());
            is_command_found = true;
        }
        
        // check if its a target
        if !targets.is_empty() && !is_command_found {
            l.log(&format!("looking for target: {}", single_argument));
            let targets = targets.get(&single_argument);
            for single_target in targets {
                let command_name = single_target.command.clone();
                if let Some(target_command) = commands.get(&command_name) {
                    l.log(&format!("found target: {}", single_argument));
                    play.commands.push(target_command.clone());
                } else {
                    println!(
                        "ERR: 64B2D565-8345-4108-B790-25606C2128C0, target not found: {}, while traversing Targets:",
                        command_name
                    );
                    process::exit(1);
                }
            }
        } else {
            help_menu.show(&init_data);
        }
    }

    result.add_play(play);
    l.log("dump: A0ED3871-1622-4D93-BCBF-1924CE2828A9");
    dbg!(&result);

    result
}