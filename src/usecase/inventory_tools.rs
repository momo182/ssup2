
use crate::usecase::network_host_utils::*;
use crate::entity::supfile::networks::{Network, Host};
use crate::entity::const_values::{TUBE_NAME_SEPARATOR, PASS_SEPARATOR};
use std::process::{Command, Stdio, Output};
use std::io::{self};
use std::collections::HashMap;
use crate::gateways::logger::Logger;
use thiserror::Error;
use std::{fmt,env};


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
    l.log(format!("host as string: {}", host).as_str());

    // Step 1: Extract field positions
    let (password_start, password_end, tube_name_start, tube_name_end) = 
        extract_field_positions(host, &l);

    // Step 2: Parse host details
    let (new_host, password, tube) = parse_host_details(
        host, 
        password_start, 
        password_end, 
        tube_name_start, 
        tube_name_end, 
        &l
    );

    // Step 3: Build initial Host structure
    let mut host_details = build_initial_host(new_host, password, tube);

    // Step 4: Handle shell-based password resolution
    resolve_shell_password(&mut host_details, &l);

    // Step 5: Process user and host information
    process_user_and_host(&mut host_details, &l);

    // Final log and return
    l.log(format!("CheckHostsForm: dump: 3DB74440-E5D9-4BEE-89D8-9C4EEB1459A9, {:?}", host_details).as_str());
    l.log("CheckHostsForm: finished checking nets");
    host_details
}

// Helper functions

fn extract_field_positions(host: &str, logger: &Logger) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
    let (password_start, mut password_end, tube_name_start, tube_name_end) = get_field_positions_from_raw_input(host);
    logger.log("dump: DE462638-4225-44C6-852F-4F20AEEC2A0D");
    logger.log(format!(
        r#"
        positions for fields:
        password_start: {:?}
        password_end: {:?}
        tube_name_start: {:?}
        tube_name_end: {:?}"#,
        password_start, password_end, tube_name_start, tube_name_end
    ).as_str());
    (password_start, password_end, tube_name_start, tube_name_end)
}

fn parse_host_details(
    host: &str,
    password_start: Option<usize>,
    mut password_end: Option<usize>,
    tube_name_start: Option<usize>,
    tube_name_end: Option<usize>,
    logger: &Logger
) -> (String, String, String) {
    let mut new_host = String::new();
    let mut password = String::new();
    let mut tube = String::new();

    if password_start.is_none() && tube_name_start.is_none() {
        logger.log("CheckHostsForm: no pass and tube found");
        new_host = host.to_string();
    }

    if let Some(pass_start) = password_start {
        logger.log("CheckHostsForm: password found");
        new_host = host[..pass_start - PASS_SEPARATOR.len()].to_string();

        if tube_name_start.is_some() {
            logger.log("CheckHostsForm: password > tube name found");
            password_end = Some(tube_name_start.unwrap() - TUBE_NAME_SEPARATOR.len());
        } else {
            logger.log("CheckHostsForm: password > no tube");
            password_end = Some(host.len());
        }

        logger.log("CheckHostsForm: done checking pass");
        password = host[pass_start..password_end.unwrap()].to_string();
    }

    if let Some(tube_name_start_pos) = tube_name_start {
        logger.log("CheckHostsForm: tube found");
        if new_host.is_empty() {
            new_host = host[..tube_name_start_pos - TUBE_NAME_SEPARATOR.len()].to_string();
        }
        tube = host[tube_name_start_pos..tube_name_end.unwrap()].to_string();
    }

    (new_host, password, tube)
}

fn build_initial_host(new_host: String, password: String, tube: String) -> Host {
    Host {
        host: new_host,
        user: None,
        pass: if password.is_empty() { None } else { Some(password) },
        tube: if tube.is_empty() { None } else { Some(tube) },
        env: None,
        sudo: false,
    }
}

fn resolve_shell_password(host_details: &mut Host, logger: &Logger) {
    if let Some(pass) = &host_details.pass {
        if is_shell(pass.as_ref()) {
            match resolve_shell(pass.as_ref()) {
                Ok(resolved_pass) => {
                    host_details.pass = Some(resolved_pass);
                }
                Err(e) => {
                    logger.log(format!("CheckHostsForm: 765212F7-64B0-4974-9A50-E8B8C1807FFE: resolving password via shell, error: {}", e).as_str());
                }
            }
        }
    }
}

fn process_user_and_host(host_details: &mut Host, logger: &Logger) {
    if host_details.host.contains("@") {
        let parts: Vec<&str> = host_details.host.split("@").collect();
        if parts.len() != 2 {
            logger.log(format!("CheckHostsForm: invalid @ count {}", parts.len()).as_str());
            return;
        }

        if (host_details.pass.is_none() || host_details.user.is_none()) && parts[0].contains(":") {
            let username = parts[0].split(":").collect::<Vec<&str>>()[0];
            let password = parts[0].split(":").collect::<Vec<&str>>()[1];

            if !username.is_empty() {
                host_details.user = Some(username.to_string());
            }

            if !password.is_empty() {
                host_details.pass = Some(password.to_string());
            }
        } else {
            host_details.user = Some(parts[0].to_string());
        }

        host_details.host = parts[1].to_string();
    }
}

fn get_field_positions_from_raw_input(host: &str) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
    let password_start = find_password_start(host);
    // can be optional
    let password_end = find_password_end(host);
    // can be optional
    let tube_name_start = find_tube_name_start(host);
    // can be optional
    let tube_name_end = find_tube_name_end(host);
    // just returns the len in chars
    (password_start, password_end, tube_name_start, tube_name_end)
}

pub fn is_shell(cmd: &str) -> bool {
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
    Ok(clean.trim().to_string())
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

/// This function is used to resolve the path to a file given
pub fn resolve_path(path: &str) -> String {
    let l = Logger::new("uc::parse_network::resolve_path");
    let mut final_path = "".to_string();
    l.log(format!("resolving path {:?}", path).as_str());

    if path.starts_with(".") {
        // get current working directory
        let curr_dir = std::env::current_dir().unwrap().to_string_lossy().to_string();

        //concatenate current working directory with path, leaving out the .
        if path[1..].to_string() == "" {
            final_path = format!("{}", curr_dir);
        } else {
            final_path = format!("{}{}", curr_dir, path[1..].to_string());
        }
    }

    // if path starts with ~/ then replace it with the home directory
    if path.starts_with("~/") {
        // get home directory
        let home_dir = std::env::var("HOME").unwrap();
        // replace ~/ with the home directory
        final_path = path.replace("~", home_dir.as_str()).to_string();
    }

    l.log(format!("final_path {:?}", final_path).as_str());
    final_path
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_path() {
        let path = "~/Documents/test.txt";
        let resolved_path = resolve_path(path);
        assert_eq!(resolved_path, "/Users/k.pechenenko/Documents/test.txt");

        let path = ".";
        let resolved_path = resolve_path(path);
        assert_eq!(resolved_path, "/Users/k.pechenenko/git/ssup2");

        let path = "./Cargo.lock";
        let resolved_path = resolve_path(path);
        assert_eq!(resolved_path, "/Users/k.pechenenko/git/ssup2/Cargo.lock");
    }

    #[test]
    fn test_check_hosts_form() {
        let host = "192.168.1.1";
        let result = check_hosts_form(host);
        assert_eq!(
            (result.host.as_str(), result.user, result.pass, result.tube),
            ("192.168.1.1", None, None, None)
        );

        let host = "192.168.1.1 | super_secret_pass";
        let result = check_hosts_form(host);
        assert_eq!(
            (result.host.as_str(), result.user, result.pass, result.tube),
            ("192.168.1.1", None, Some("super_secret_pass".to_string()), None)
        );

        let host = "192.168.1.1 | super_secret_pass > tubes";
        let result = check_hosts_form(host);
        assert_eq!(
            (result.host.as_str(), result.user, result.pass, result.tube),
            ("192.168.1.1", None, Some("super_secret_pass".to_string()), Some("tubes".to_string()))
        );


        let host = "root@192.168.1.1 | super_secret_pass > tubes";
        let result = check_hosts_form(host);
        assert_eq!(
            (result.host.as_str(), result.user, result.pass, result.tube),
            ("192.168.1.1", Some("root".to_string()), Some("super_secret_pass".to_string()), Some("tubes".to_string()))
        );

        // test if we have a password as plain text and a password via separator,
        // but the password via plain text takes precedence over the password as password via separator
        // as the intended way is to use plaintext passwords as fallback for quick checks only.
        let host = "root:super_secret_pass@192.168.1.1 | foobar > tubes";
        let result = check_hosts_form(host);
        assert_eq!(
            (result.host.as_str(), result.user, result.pass, result.tube),
            ("192.168.1.1", Some("root".to_string()), Some("super_secret_pass".to_string()), Some("tubes".to_string()))
        );

        let host = "root:super_secret_pass@192.168.1.1 | $(echo barfoo22) > tubes1";
        let result = check_hosts_form(host);
        assert_eq!(
            (result.host.as_str(), result.user, result.pass, result.tube),
            ("192.168.1.1", Some("root".to_string()), Some("super_secret_pass".to_string()), Some("tubes1".to_string()))
        );
    }
}