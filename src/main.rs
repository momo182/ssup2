use std::env::args;

use crate::usecase::program_init;
use clap::Parser;
use entity::playbook;
use gateways::logger::Logger;
use usecase::program_init::parse_initial_args;
mod entity;
mod usecase;
mod gateways;

fn process_flags() -> entity::CommandLineArgs {
    let args = entity::CommandLineArgs::parse();
    return args;
}


fn main() {
    let l = Logger::new("main");
    // collect arguments into a vector
    let mut args: Vec<String> = args().collect();

    _ = args.remove(0);
    l.log(format!("drop binary name from args: {:?}", &args));

    let flags = process_flags();
    // println!("args are {:?}", &args);
    l.log("reading supfile");
    let supfile = program_init::parse_supfile(flags.clone());
    // dbg!(supfile.clone());

    // find full path to supfile
    let start_state = entity::InitState{
        args: args,
        supfile: supfile,
        flags: flags,
        make_mode: false,
    };
    l.log("finished reading supfile");
    l.log("getting a playbook");
    let playbook: playbook::PlayBook = parse_initial_args(&mut start_state.clone());
    let is_makefile_mode = playbook.is_makefile;

    for play in playbook.get_plays() {
        let networks = play.nets.clone();
        let commands = play.commands.clone();

        // negative checks here
        if networks.len() == 0 {
            println!("No networks found for this play");
        } 
        if commands.len() == 0 {
            println!("No commands found for this play");
        }

        dbg!(commands);
        dbg!(networks);
        dbg!(is_makefile_mode);


    } // end of for play in playbook

    l.log("end of main");
}
