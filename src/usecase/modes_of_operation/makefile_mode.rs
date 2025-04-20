use crate::entity::help_displayer::HelpDisplayer;
use crate::entity::playbook::{Play, PlayBook};
use crate::usecase::ensure_localhost;
use crate::gateways::logger::logger_func as l;
use crate::entity::InitState;


#[allow(unused_imports)]
#[allow(dead_code)]
pub fn makefile_mode(init_data: &mut InitState, help_menu: &mut HelpDisplayer) -> PlayBook {
    let mut result = PlayBook::new();
    let mut play = Play::new();
    ensure_localhost(init_data);
    l("makefile mode");
    // get localhost network
    let networks = init_data.supfile.networks.clone();
    let targets = init_data.supfile.targets.clone();
    let commands = init_data.supfile.commands.clone();
    let localhost_network = networks.get("localhost").expect("C5A59F87-CB34-4660-B527-6FD1CCAAA144: localhost network not found, but must be present");
    result.mark_as_makefile_mode();

    play.add_net(localhost_network.clone());
    for single_argument in init_data.args.iter() {
        let mut is_command = false;
        let mut is_target = false;

        if targets.has(single_argument) {
            is_target = true;
        }

        if init_data.supfile.commands.is_empty() {
            l("no commands found in supfile");
        }

        if commands.contains_key(single_argument) {
            is_command = true;
        }

        // if no target nor command is set, we need to print help menu
        if !is_command && !is_target {
            help_menu.show_cmd = true;
            help_menu.show_networks = true;
            help_menu.show(&init_data.clone());
        }

        if is_command {
            let command = commands.get(&single_argument.clone()).unwrap();
            play.add_command(command.clone());
        }

        if is_target {
            let supfile_targets = init_data.supfile.targets.clone();
            let targets = supfile_targets.get(&single_argument.clone());
            for target in targets {
                let command_name = target.command;
                let command = commands.get(&command_name).unwrap();
                play.add_command(command.clone());
            }
        }
    }

    result.add_play(play);
    result
}