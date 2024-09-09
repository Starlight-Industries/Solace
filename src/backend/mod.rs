
use colored::Colorize;
use dioxus::html::tr;
use serde::Serialize;
use std::{env::{self, consts::OS}, fs, os, path::PathBuf};
mod init;
enum Loader {
    Vanilla,
    Forge,
    Fabric,
    Quilt,
    Paper,
    Spigot,
    Bukkit,
    Purpur,
    Sponge,
    Bungee,
    Velocity,
    Folia, // nobody is gonna use folia lol
    Mohist // Nobody is gonna use mohist either
}

#[derive(Serialize)]
pub(crate) struct Server {
    name: String,
    port: u16,
    is_running: bool,
    initalized: bool,
}

impl Server {
    pub fn construct(name: &str, port: u16) -> Self {
        println!("Constructing server {} with port {}",name.blue(), port.to_string().green(),);
        return Self {
            name: name.to_string(),
            port,
            is_running: false,
            initalized: false,
        };
    }
    pub fn start_server(&mut self) {
        if !self.is_running && self.initalized {
            println!("starting {} on {}", self.name.blue(),self.port.to_string().green())
        } else {
            println!("{} server: {} already started","Err:".red(),self.name)
        }
    }

    pub fn init(&mut self) -> std::io::Result<()> {
        let current_path = env::current_dir()?;
        let dir: &str = &format!("./.solace/servers/{}",& mut self.name.red());
        let _ = fs::create_dir_all(dir);
        println!("Initalizing {} at {}.",self.name,dir);
        self.initalized = true;
        Ok(())
    }
}