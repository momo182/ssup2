use std::env::args;

use crate::usecase::program_init;
use clap::Parser;
use entity::playbook;
use gateways::logger::Logger;
use gateways::shellcheck::run_shellcheck;
use usecase::program_init::{check_additional_flags, parse_initial_args, merge_vars};
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
    let mut init_state: entity::InitState = entity::InitState{
        args: args,
        supfile: supfile,
        flags: flags,
        make_mode: false,
    };

    l.log("finished reading supfile");
    l.log("getting a playbook");
    let playbook: playbook::PlayBook = parse_initial_args(&mut init_state.clone());
    let is_makefile_mode = playbook.is_makefile;

    for play in playbook.get_plays() {
        let mut network = play.network.clone();
        let commands = play.commands.clone();

        // negative checks here
        if commands.len() == 0 {
            println!("No commands found for this play");
        }

        run_shellcheck(init_state.clone());
        check_additional_flags(&mut network, &mut init_state);
        let merged_vars = merge_vars(&mut network, &mut init_state);

        dbg!(commands);
        dbg!(network);
        dbg!(is_makefile_mode);


    } // end of for play in playbook

    l.log("end of main");
}
