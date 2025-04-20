use std::process::Command as OsCmd; 
use super::supfile::Supfile;
use super::InitState;
use crate::gateways::logger::logger_func as l;

pub struct HelpDisplayer {
    pub show_networks: bool,
    pub show_targets: bool,
    pub show_cmd: bool,
    pub show_make_mode: bool,
    pub color: bool,
}

impl HelpDisplayer {
    pub fn new(init: InitState) -> HelpDisplayer {
        let hd = HelpDisplayer {
            show_networks: false,
            show_cmd: false,
            show_targets: false,
            show_make_mode: init.make_mode,
            color: init.flags.disablecolor,
        };
        hd
    }

    pub fn show(&self, state: &InitState) {
        let conf = &state.supfile;
        if self.color {
            self.print_bw_help(&conf);
        } else {
            self.print_bw_help(&conf);
        }
    }

    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fn show_all(&mut self, state: &InitState) {
        self.show_networks = true;
        self.show_cmd = true;
        self.show_targets = true;
        self.show(state);
    }


    fn print_bw_help(&self, conf: &Supfile) {
        if self.show_make_mode {
            println!("No networks defined, makefile mode available");
        }

        self.print_out_mods_status();
        
        if self.show_networks {
            self.network_usage(conf);
        }

        if self.show_cmd {
            self.cmd_usage(conf);
        }

        if self.show_targets {
            self.target_usage(conf);
        }
    }

    fn target_usage(&self, conf: &Supfile) {
        println!("Targets:");
        let targets = conf.targets.clone();
        let target_names = targets.names;
        for target_name in target_names {
            l(format!("getting affix for target name: {}", target_name).as_str());
            let affixed_targets = match targets.targets.get(&target_name) {
                Some(affixed_targets) => {
                    affixed_targets.clone()
                },
                None => {
                    panic!("74BB28F7-ADAB-4478-BB20-87EA04746575: failed to get affixed_targets for {}", target_name)
                },
            };

            for affixed_target in affixed_targets {
                println!("- {} -> {}", affixed_target.command, affixed_target.affixed_network);
            }

        }
        
    }



    fn cmd_usage(&self, conf: &Supfile) {
        println!("Commands:");
        let cmd_names = conf.commands.keys();
        for name in cmd_names {
            if let Some(cmd) = conf.commands.get(name) {
                println!("- {}: {}", name, cmd);
            }
        }
    }

    fn network_usage(&self, conf: &Supfile) {
        println!("Networks:");
        let networks = conf.networks.clone();
        let net_names = networks.nets.keys();
        for name in net_names {
            println!("- {}", name);
            if let Some(network) = networks.get(name) {
                let hosts = &network.hosts;
                    for host in hosts {
                        println!("  - {}", host);
                    }
            }
        }
    }

    fn print_out_mods_status(&self) {
        let is_shellcheck_installed = OsCmd::new("shellcheck").arg("-V").output().is_ok();
        let shellcheck_sign = if is_shellcheck_installed { "✓" } else { "✖️" };
        println!("{} shellcheck", shellcheck_sign);
    }
}
