use crate::entity::supfile::networks::Network;
use crate::entity::supfile::commands::Command;

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone, Default)]
pub struct Play {
    pub network: Network,
    pub commands: Vec<Command>,
}

impl Play {
    pub fn new() -> Self {
        Self {
            network: Network::default(),
            commands: Vec::new(),
        }
    }

    pub fn add_net(&mut self, network: Network) {
        self.network = network;
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

}