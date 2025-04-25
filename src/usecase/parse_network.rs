
use crate::usecase::network_host_utils::*;
use crate::entity::supfile::networks::{Network, HostEntry, Host};
use crate::entity::const_values::TUBE_NAME_SEPARATOR;
use std::process::{Command, Stdio, Output};
use std::io::{self};
use std::collections::HashMap;
use crate::gateways::logger::Logger;
use thiserror::Error;
use std::{fmt,env};


// #[derive(Error, Debug)]
// enum ParseError {
//     #[error("Missing inventory field in Supfile")]
//     MissingInventory,
// }


#[derive(Error, Debug)]
pub enum ShellResolveError {
    #[error("Missing $( prefix")]
    MissingPrefix,
    #[error("Missing ) suffix")]
    MissingSuffix,
    #[error("Failed to run command: {cmd}")]
    CommandExecution {
        cmd: String,
        #[source]
        source: io::Error,
        trace_id: &'static str,
    },
}

impl ShellResolveError {
    fn command_error(cmd: &str, source: io::Error) -> Self {
        ShellResolveError::CommandExecution {
            cmd: cmd.to_string(),
            source,
            trace_id: "6928F3B4-0D17-45FB-9633-DABA63E163A1",
        }
    }
}


pub fn parse_inventory(network: &Network) -> Result<Vec<Host>,std::io::Error> {
    let inventory_data;
    let network_env: HashMap<String,String>;

    let network_env_unpacked = network.env.clone();
    let mut new_env = env::vars().collect::<HashMap<String, String>>();
    new_env.extend(network_env_unpacked.iter().map(|(k, v)| (k.clone(), v.clone())));
    network_env = network_env_unpacked.clone();

    let inventory = &network.inventory;
    inventory_data = inventory.clone();

    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(inventory_data)
        .envs(&network_env)
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed with status: {}", output.status),
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let hosts: Vec<Host> = output_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| check_hosts_form(line))
        .collect();
    Ok(hosts)
}



pub fn check_hosts_form(host: &str) -> Host {
    let l = Logger::new("uc::parse_network::check_hosts_form");
    let mut host_details = Host {
            host: "".to_string(),
            user: None,
            pass: None,
            tube: None,
            env: None,
            sudo: false
        };
    
    l.log( format!("host as string: {}", host).as_str());

    let password_start = find_password_start(host); // can be optional
    let mut password_end = find_password_end(host); // can be optional
    let tube_name_start = find_tube_name_start(host); // can be optional
    let tube_name_end = find_tube_name_end(host); // just returns the len in chars

    l.log("dump: DE462638-4225-44C6-852F-4F20AEEC2A0D");
    l.log(format!( r#"
CheckHostsForm:
password_start: {:?}
password_end: {:?}
tube_name_start: {:?}
tube_name_end: {:?}"#, 
password_start, 
password_end, 
tube_name_start, 
tube_name_end).as_str());

    let mut new_host = String::new();
    let mut password = String::new();
    let mut tube = String::new();

    if password_start == None && tube_name_start == None {
        l.log( "CheckHostsForm: no pass and tube found");
        new_host = host.to_string();
    }




    if let Some(pass_start) = password_start {
        l.log("CheckHostsForm: password found");
        // let new_host_start = pass_start - PASS_SEPARATOR.len();
        new_host = host[..].to_string();

        if tube_name_start != None {
            l.log("CheckHostsForm: password > tube name found");
            password_end = Some(tube_name_start.unwrap() - TUBE_NAME_SEPARATOR.len());
        } else {
            l.log("CheckHostsForm: password > no tube");
            password_end = Some(host.len());
        }

        l.log("CheckHostsForm: done checking pass");
        password = host[pass_start..password_end.unwrap()].to_string();
    }

    if let Some(tube_name_start_pos) = tube_name_start {

        l.log("CheckHostsForm: tube found");
        if new_host.is_empty() {
            new_host = host[..tube_name_start_pos - TUBE_NAME_SEPARATOR.len()].to_string();
        }
        tube = host[tube_name_start_pos..tube_name_end.unwrap()].to_string();
    }

    host_details.host = new_host;
    host_details.pass = Some(password.to_string());
    host_details.tube = Some(tube.to_string());

    if is_shell(&host_details.pass.clone().unwrap().as_ref()) {
        match resolve_shell(&host_details.pass.clone().unwrap()) {
            Ok(pass) => {
                host_details.pass = Some(pass);
            }
            Err(e) => {
                l.log(format!("CheckHostsForm: 765212F7-64B0-4974-9A50-E8B8C1807FFE: resolving password via shell, error: {}", e).as_str());
            }
        }
    }

    l.log(format!("CheckHostsForm: dump: 3DB74440-E5D9-4BEE-89D8-9C4EEB1459A9, {:?} ", host_details).as_str());
    l.log("CheckHostsForm: finished checking nets ");
    host_details
}


fn is_shell(cmd: &str) -> bool {
    cmd.starts_with("$(") && cmd.ends_with(')')
}

pub fn resolve_shell(value: &str) -> Result<String, ShellResolveError> {
    let l = Logger::new("uc::parse_network::resolve_shell");
    // remove the prefix and suffix
    let value = value
        .strip_prefix("$(")
        .ok_or(ShellResolveError::MissingPrefix)?
        .strip_suffix(')')
        .ok_or(ShellResolveError::MissingSuffix)?;

    l.log(format!("to run command {:?}", value).as_str());
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(value)
        .stderr(io::stderr())
        .output()
        .map_err(|e| ShellResolveError::command_error(value, e))?;

    // limit value to only printable characters
    let clean = filter_non_printable(&output.stdout);
    l.log(format!("cmd dump: {}\nvalue: {}", format_output(&output), clean).as_str());
    Ok(clean)
}






fn filter_non_printable(input: &[u8]) -> String {
    let mut filtered = String::new();
    for byte in input {
        if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
            filtered.push(*byte as char);
        }
    }
    filtered
}





// Helper function to format command output
fn format_output(output: &Output) -> impl fmt::Display + '_ {
    struct OutputFormatter<'a>(&'a Output);
    
    impl<'a> fmt::Display for OutputFormatter<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "status: {}, stdout: {:?}, stderr: {:?}",
                self.0.status,
                String::from_utf8_lossy(&self.0.stdout),
                String::from_utf8_lossy(&self.0.stderr)
            )
        }
    }
    
    OutputFormatter(output)
}
