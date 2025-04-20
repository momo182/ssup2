// Environment variable constants
pub const CSUP_PASSWD_ENV: &str = "SUP_PASSWORD";
pub const CSUP_DO_SUDO_ENV: &str = "SUP_SUDO";

// Separator constants
pub const PASS_SEPARATOR: &str = " | ";
pub const TUBE_NAME_SEPARATOR: &str = " << ";

// Other constants
pub const MAIN_SCRIPT: &str = "_ssup_run";
pub const VARS_FILE: &str = "_ssup_env";
pub const HASHED_PASS: &str = "_ssup_pass";
pub const INJECTED_COMMANDS_FILE: &str = "_ssup_commands";
pub const SSUP_WORK_FOLDER: &str = ".local/ssup/run/";
pub const VERSION: &str = "0.5";
pub const SOURCE_DIRECTIVE: &str = "#source://";