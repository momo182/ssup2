use crate::entity::help_displayer::HelpDisplayer;
use crate::entity::InitState;
use std::process;
use crate::gateways::logger::logger_func as l;
use crate::entity::playbook::{PlayBook,Play};
use crate::usecase::env_parser::parse_env;
use crate::usecase::{ensure_network_exists,override_env_from_args,add_ssup_default_envs};
use crate::usecase::parse_network::parse_inventory;

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
    l("usecase::ParseInitialArgs::normalMode");

    let mut result = PlayBook { 
        plays: Vec::new(),
        is_makefile: false,
    };

    let env_from_args = parse_env(&init_data.flags.env);
    let mut args = init_data.args.clone();

    let network_name = args.remove(0);

    ensure_network_exists(&network_name, &init_data, &help_menu);

    let mut network = init_data.supfile
        .networks
        .clone()
        .nets
        .get(&network_name)
        .cloned()
        .expect(format!("Network '{}' not found", network_name).as_str());

    l("parse CLI --env flag env vars, override values defined in Network env");
    override_env_from_args(&env_from_args, &mut network);

    l("check if we have an inventory via script execution");
    let hosts = parse_inventory(&network).expect("failed to parse inventory from script");
    network.hosts.extend(hosts);

    add_ssup_default_envs(&mut network, &init_data);

    let mut play = Play {
        nets: Some(network),
        commands: Vec::new(),
    };


    for single_argument in args {
        l(&format!("parse given command: {}", single_argument));
        l("check if its a target");
        let conf = init_data.supfile.clone();

        // check if its a command
        if let Some(command) = conf.commands.get(&single_argument) {
            l(&format!("found command: {}", single_argument));
            play.commands.push(command.clone());
        }
        
        // check if its a target
        let targets = conf.targets.clone();
        if !targets.is_empty() {
            l(&format!("found target: {}", single_argument));
            let targets = targets.get(&single_argument);
            for single_target in targets {
                let command_name = single_target.command.clone();
                if let Some(target_command) = conf.commands.get(&command_name) {
                    play.commands.push(target_command.clone());
                } else {
                    eprintln!(
                        "ERR: 64B2D565-8345-4108-B790-25606C2128C0, command not found: {}, while traversing Targets:",
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
    l("dump: A0ED3871-1622-4D93-BCBF-1924CE2828A9");
    l(&format!("{:?}", &init_data));

    result
}