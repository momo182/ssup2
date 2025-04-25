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

#[derive(Debug, Clone)]
pub struct Play {
    pub nets: Vec<Network>,
    pub commands: Vec<Command>,
}

impl Play {
    pub fn new() -> Self {
        Self {
            nets: Vec::new(),
            commands: Vec::new(),
        }
    }

    pub fn add_net(&mut self, nets: Network) {
        if self.nets.is_empty() {
            self.nets.push(nets);
        } else {
            // checking for duplicate nets
            let mut exist = false;
            for net in self.nets.iter() {
                if net.name == nets.name {
                    exist = true;
                    break;
                }
            }
            if !exist {
                self.nets.push(nets);
            }
        }
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

}