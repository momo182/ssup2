use std::env::args;

use crate::usecase::program_init;
use crate::entity::help_displayer::HelpDisplayer;
use clap::Parser;
mod entity;
mod usecase;
mod gateways;

fn process_flags() -> entity::CommandLineArgs {
    let args = entity::CommandLineArgs::parse();
    return args;
}


fn main() {
    // collect arguments into a vector
    let args: Vec<String> = args().collect();
    let flags = process_flags();
    // println!("args are {:?}", &args);
    let supfile = program_init::parse_supfile(flags.clone());
    dbg!(supfile.clone());

    // find full path to supfile
    let start_state = entity::InitState{
        args: args,
        supfile: supfile,
        flags: flags,
        make_mode: false,
    };
    let mut help_displayer = HelpDisplayer::new(start_state.clone());
    help_displayer.show_cmd = true;
    help_displayer.show_networks = true;
    help_displayer.show_targets = true;
    help_displayer.show(&start_state.clone());
    
}
