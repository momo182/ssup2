use crate::entity::supfile::networks::Network;
use crate::entity::supfile::commands::Command;
use crate::gateways::logger::logger_func as l;


pub struct PlayBook {
    pub plays: Vec<Play>,
    pub is_makefile: bool,
}

impl PlayBook {
    pub fn new() -> Self {
        Self {
            plays: Vec::new(),
            is_makefile: false,
        }
    }

    pub fn add_play(&mut self, play: Play) {
        self.plays.push(play);
    }

    pub fn get_plays(&self) -> &Vec<Play> {
        &self.plays
    }

    pub fn mark_as_makefile_mode(&mut self) {
        self.is_makefile = true;
    }

    pub fn is_makefile_mode(&self) -> bool {
        self.is_makefile
    }
}

pub struct Play {
    pub nets: Option<Network>,
    pub commands: Vec<Command>,
}

impl Play {
    pub fn new() -> Self {
        Self {
            nets: None,
            commands: Vec::new(),
        }
    }

    pub fn add_net(&mut self, nets: Network) {
        if self.nets.is_none() {
            self.nets = Some(nets);
        }
        else {
            l("network is already set!");
        }
    }

    pub fn get_net(&self) -> &Option<Network> {
        &self.nets
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn get_commands(&self) -> &Vec<Command> {
        &self.commands
    }
}