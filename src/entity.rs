pub mod const_values;
pub mod supfile;
pub mod help_displayer;
pub mod playbook;
pub mod env;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
//&init_data.args.command_args;

pub struct CommandLineArgs {
    #[arg(short = 'f', default_value = "", required = false)]
    pub file: String,
    #[arg(short = 'e', long = "env", required = false, value_parser, num_args = 1..)]
    pub env: Vec<String>,
    #[arg(short = 'D', long = "debug", required = false, default_value = "false")]
    pub debug: bool,
    #[arg(long = "sshconfig", required = false, default_value = "")]
    pub sshconfig: String,
    #[arg(short = 'c', long = "no-color", required = false, default_value = "false")]
    pub disablecolor: bool,
    #[arg( long = "disable-prefix", required = false, default_value = "false")]
    pub disableprefix: bool,
    #[arg( long = "except", required = false, default_value = "")]
    pub excepthosts: String,
    #[arg( long = "only", required = false, default_value = "")]
    pub onlyhosts: String,
    #[arg(trailing_var_arg = true)]
    pub extra_args: Vec<String>,
}

impl CommandLineArgs {
    pub fn parse_env(&self) -> std::collections::HashMap<String, String> {
        self.env.iter()
            .filter_map(|s| {
                let parts: Vec<&str> = s.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect()
    }
}


#[derive(Clone, Debug)]
pub struct InitState {
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub args: Vec<String>,
    pub flags: CommandLineArgs,
    pub supfile: supfile::Supfile,
    pub make_mode: bool,
}

