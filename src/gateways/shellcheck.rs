use crate::gateways::logger::Logger;
use crate::entity::InitState;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Write;
use std::process::{Command, Stdio};

fn look_path(program: &str) -> Option<PathBuf> {
    // grab the PATH environment variable
    if let Some(paths) = env::var_os("PATH") {
        // split paths
        for path in env::split_paths(&paths) {
            // form the full path to the candidate
            let candidate = path.join(program);

            // check if the candidate exists and is executable
            if candidate.exists() && is_executable(&candidate) {
                return Some(candidate);
            }
        }
    }
    None
}


#[cfg(target_family = "unix")]
fn is_executable(path: &PathBuf) -> bool {
    use std::os::unix::fs::MetadataExt;
    if let Ok(metadata) = fs::metadata(path) {
        metadata.mode() & 0o111 != 0 // check executable bit
    } else {
        false
    }
}

#[cfg(target_family = "windows")]
fn is_executable(_path: &PathBuf) -> bool {
    true // assume it is executable on Windows
}


pub trait ShellCheckFacade {
    fn check(&self, content: &str, command_name: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct ShellCheckProvider;

impl ShellCheckFacade for ShellCheckProvider {
    fn check(&self, content: &str, command_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let l = Logger::new("gw::shellcheck::check");
        l.log(format!("Running shellcheck on command: {}", command_name));

        // check if shellcheck is installed at all
        if let Some(sc_path) = look_path("shellcheck") {
            l.log(format!("Found shellcheck at: {:?}", sc_path));
        } else {
            l.log(format!("shellcheck not found in PATH"));
            return Err("Shellcheck not found in PATH. Please install it and try again.".to_string().into());
        }
        l.log(format!("command contend:\n{:?}", content));

        let check_args = vec![
            "-f", "tty",
            "-Calways",
            "-e", "SC2148,SC2155,SC2001",
            "-",
        ];

        // Создаем команду shellcheck
        let mut sc_command = Command::new("shellcheck")
            .args(&check_args)
            .stdin(Stdio::piped()) // redirect stdin
            .stdout(Stdio::piped()) // redirect stdout
            .stderr(Stdio::piped()) // redirect stderr
            .spawn()?; // run the process

        if let Some(mut stdin) = sc_command.stdin.take() {
            stdin.write_all(content.as_bytes())?;
        }

        let output = sc_command.wait_with_output()?;
        let combined_output = String::from_utf8(output.stdout)?;
        l.log(format!("{}", combined_output).as_str());

        // return error if shellcheck exited with error
        if output.status.success() {
            return Ok(())
        } else {
            let exit_code = output.status.code().expect("exit code must exist, wtf");
            println!("exit code: {:?}", exit_code);
            return Err("shellsheck exited with error".into());
        }
    }
}

pub fn run_shellcheck(init_state: InitState) {
    let l = Logger::new("gw::shellcheck::run_shellcheck");
    l.log("Will run shellcheck on Supfile");

    let shellcheck = ShellCheckProvider;
    let mut errors: Vec<String> = Vec::new();
    let sup_file = init_state.supfile;
    let command_names = &sup_file.commands;

    for (command_name, command_content) in command_names {
        l.log(&format!("run shellcheck on command: {:?}", command_name));

        let trimmed_run = command_content.run.trim();
        let trimmed_local = command_content.local.trim();
        let trimmed_script = command_content.script.trim();

        l.log(&format!("len command.Run: {}", trimmed_run.len()));
        l.log(&format!("len command.Local: {}", trimmed_local.len()));

        if !trimmed_run.is_empty() {
            l.log(&format!("Will run shellcheck on command: {}", trimmed_run));

            let first_line = trimmed_run.lines().next().unwrap_or("");
            if should_skip_shellcheck(first_line) {
                continue;
            }

            if let Err(e) = shellcheck.check(trimmed_run, command_name) {
                errors.push(e.to_string());
            }
        } else {
            l.log("command.Run is empty, skipping shellcheck");
        }

        if !trimmed_script.is_empty() {
            l.log(&format!("Will run shellcheck on script: {}", trimmed_script));

            let first_line = trimmed_script.lines().next().unwrap_or("");
            if should_skip_shellcheck(first_line) {
                continue;
            }

            if let Err(e) = shellcheck.check(trimmed_script, command_name) {
                errors.push(e.to_string());
            }
        } else {
            l.log("command.Script is empty, skipping shellcheck");
        }

        if !trimmed_local.is_empty() {
            l.log(&format!("Will run shellcheck on local: {}", trimmed_local));

            let first_line = trimmed_local.lines().next().unwrap_or("");
            if should_skip_shellcheck(first_line) {
                continue;
            }

            if let Err(e) = shellcheck.check(trimmed_local, command_name) {
                errors.push(e.to_string());
            }
        } else {
            l.log("command.Local is empty, skipping shellcheck");
        }
    }

    if !errors.is_empty() {
        let last_error = errors.last().unwrap();
        println!("Last error from shellcheck: {}", last_error);
    }

}

fn should_skip_shellcheck(first_line: &str) -> bool {
    // TODO reverse the logic here
    // only do shellcheck if shell in shebang is compatible with shellcheck

    let l = Logger::new("gw::shellcheck::should_skip_shellcheck");
    if first_line.contains("fish") {
        l.log("First line contains fish, skipping shellcheck");
        true
    } else if first_line.contains("nu") {
        l.log("First line contains nu shell, skipping shellcheck");
        true
    } else if first_line.contains("nosc") {
        l.log("First line contains nosc tag, skipping shellcheck");
        true
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::{ShellCheckFacade, ShellCheckProvider};

    #[test]
    fn test_shellcheck_with_unquoted_variable_error() {
        let provider = ShellCheckProvider {};
        let content = "$var=\"value\"\necho $var\n"; // Unquoted variable usage
        let result = provider.check(content, "bad_script");

        assert!(result.is_err());
    }

    #[test]
    fn test_shellcheck_with_quoted_variable_ok() {
        let provider = ShellCheckProvider {};
        let content = "var=\"value\"\necho \"$var\"\n"; // Properly quoted
        let result = provider.check(content, "good_script");

        assert!(result.is_ok());
    }
}