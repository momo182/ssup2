use crate::entity::help_displayer::HelpDisplayer;
use crate::entity::playbook::{Play, PlayBook};
use crate::entity::supfile::networks::Network;
use crate::gateways::logger::Logger;
use crate::entity::InitState;
use crate::usecase::{add_ssup_default_envs, ensure_network_exists};

#[allow(dead_code)]
#[allow(unused_imports)]
// SpecialTargetMode is a mode where all args are Target names
// and networks are actually defined inside Supfile
pub fn special_target_mode(init_state: &InitState, help_displayer: &mut HelpDisplayer) -> PlayBook {
    let l = Logger::new("uc::modes_of_operation::special_target_mode");
    l.log("special target mode");
    // targets is really a hash map
    // pub targets: Option<HashMap<String, String>>,
    let mut result = PlayBook::new();
    let targets = init_state.supfile.targets.clone();
    let mut networks = init_state.supfile.networks.clone();
    let commands = init_state.supfile.commands.clone();
    for single_arg in init_state.args.clone() {
        // try grabbin the target with the name of the current arg
        // if it fails than we have 2 fail here
        l.log(format!("argument name: {}", single_arg).as_str());

        let affixed_targets = targets.get(&single_arg);
        
        l.log(format!("{:?}", affixed_targets).as_str());
        for affixed_target in affixed_targets {
            let command_name = affixed_target.command.clone();
            let affixed_network = affixed_target.affixed_network.clone();


            let report = format!("affix: {:?}\ncommand: {}", affixed_target, command_name);
            l.log(format!("{}", report).as_str());

            ensure_network_exists(affixed_network.as_str(), init_state, help_displayer);
            let affixed_network: &mut Network = networks.get_mut(affixed_network.as_str()).expect(format!(
                "F0AABC9B-9214-444D-B864-F6AE785EA695: error getting network {}", 
                affixed_network.as_str()).as_str());
            // TODO rewrite replacement for OverrideEnvFromArgs
            // and use it here, skipped for now
            add_ssup_default_envs(affixed_network, &init_state);
            let command = commands
            .get(&command_name.clone())
            .expect("0325F68E-5D0C-45BA-970E-D97C5D38B07A3: error getting command");

            l.log(format!("command: {:?}", command).as_str());
            l.log(format!("affix: {:?}", affixed_network).as_str());

            // fill in the values for play
            let mut play = Play::new();
            play.add_command(command.clone());
            play.add_net(affixed_network.clone());

            result.add_play(play);
        }
    }
    result
        
}