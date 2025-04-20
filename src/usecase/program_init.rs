use crate::entity::supfile;
use crate::entity::help_displayer::HelpDisplayer;
use crate::entity::CommandLineArgs;
use std::env;
use std::fs;
use std::path::PathBuf;
use log::info;
use crate::entity::{InitState, playbook::PlayBook};
use crate::usecase::modes_of_operation::{special_target_mode::special_target_mode, normal_mode::normal_mode, makefile_mode::makefile_mode};
use crate::gateways::logger::logger_func as l;

pub fn parse_supfile(args: CommandLineArgs) -> supfile::Supfile {
    let mut file_to_read: std::path::PathBuf;
    let start_dir = env::current_dir().expect("failed to get current dir");
    l(&format!("supfile parsing started in folder: {:?}", start_dir));

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
    let conf = init_data.supfile.clone();
    let args = &init_data.args;
    let args_count = args.len();
    let mut help_menu = HelpDisplayer::new(init_data.clone());

    info!("Checking if we have any args at all, len: {}", args_count);

    if conf.networks.is_empty() {
        if all_args_are_targets(&init_data, &mut help_menu) {
            info!("Special target mode");
            return special_target_mode(&init_data, &mut help_menu);
        }
        
        info!("Normal mode");
        return normal_mode(&init_data, &help_menu);
    }

    info!("Makefile mode");
    makefile_mode(init_data, &mut help_menu)
}

fn all_args_are_targets(init_data: &InitState, help_menu: &mut HelpDisplayer) -> bool {
    let conf = &init_data.supfile;
    let args = &init_data.args;
    let mut no_missing_names = true;
    
    info!("Checking if all given args are targets: {}", args.len());
    
    let targets = conf.targets.clone();

    for single_argument in args {
        if !targets.has(single_argument) {
            info!("Targets check -> unknown keyword: {}", single_argument);
            no_missing_names = false;
            help_menu.show_all(&init_data);
        }
    }
    
    no_missing_names
}
