use colored::Colorize;
use serde::{Deserialize, Serialize};
pub mod installer;
use color_eyre::Result;
use inquire::{Select, Text};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use solace::{Loader, LoaderType};

use std::fs::{create_dir_all, read_to_string, write};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Server {
    name: String,
    port: u16,
    server_dir: String,
    server_loader: Loader,
    is_running: bool,
    initalized: bool,
}

impl Server {
    pub fn construct(name: &str, port: u16, loader: LoaderType) -> Result<Self> {
        println!(
            "Constructing server {} with port {}",
            name.blue(),
            port.to_string().green()
        );

        let config_path = format!("./.solace/servers/{}/server_config.toml", name);
        let content = read_to_string(&config_path)?;
        let server: Server = toml::from_str(&content)?;
        if server.name == name {
            println!("Loading existing configuration for {}", name.blue());
            return Ok(server);
        }
        Ok(Self {
            name: name.to_string(),
            port,
            server_dir: format!("./.solace/servers/{}", name),
            server_loader: Loader {
                typ: loader,
                version: "1.21.1".to_string(),
            },
            is_running: false,
            initalized: false,
        })
    }
    pub fn create_server() -> Result<()> {
        println!("Creating a new server...{}", "".green());

        let name = Text::new("Server Name: ")
            .prompt()
            .expect("Failed to get server name");

        let loader_type = Select::new(
            "Server loader",
            vec![
                LoaderType::Vanilla,
                LoaderType::Forge,
                LoaderType::Fabric,
                LoaderType::Quilt,
                LoaderType::Paper,
                LoaderType::Spigot,
                LoaderType::Bukkit,
                LoaderType::Purpur,
                LoaderType::Sponge,
                LoaderType::Bungee,
                LoaderType::Velocity,
                LoaderType::Folia,
                LoaderType::Mohist,
            ],
        )
        .prompt()?;

        let port = Text::new("Port: ").with_default("25565").prompt()?;

        let port_number: u16 = port.parse()?;

        let mut new_server = Server::construct(&name, port_number, loader_type)?;

        new_server.init()?;
        new_server.start_server()
    }

    pub fn start_server(&mut self) -> Result<()> {
        if !self.is_running && self.initalized {
            println!(
                "starting {} on {}",
                self.name.blue(),
                self.port.to_string().green()
            );

            let file_path = "./server.jar";
            let config_file = format!("{}/server_config.toml", self.server_dir);

            println!("{config_file}");

            let content = read_to_string(&config_file)?;
            let mut config: Server = toml::from_str(&content)?;
            config.is_running = true;
            let new_content = toml::to_string(&config)?;

            println!("{:?}", config);

            write(format!("{}", config_file.clone()), new_content)?;

            self.handle_termination()?;

            let mut child = Command::new("java")
                .arg("-jar")
                .arg(file_path)
                .arg("-nogui")
                .current_dir(self.server_dir.clone())
                //.stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to launch server");

            loop {
                match child.try_wait()? {
                    Some(status) => {
                        println!("Server process exited with status: {}", status);
                        self.exit()?;
                        break;
                    }
                    None => {
                        //TODO: Make a less lazy implementation of checking if the server is still running
                        sleep(Duration::from_secs(30));
                    }
                }
            }
        } else if !self.is_running && !self.initalized {
            println!("Server not initalized. Finishing process");
            self.init()?;
        } else {
            println!("{} server: {} already started", "Err:".red(), self.name);
        }
        Ok(())
    }
    pub fn init(&mut self) -> Result<()> {
        //let current_path = env::current_dir();
        let dir = &format!("./.solace/servers/{}", &mut self.name);
        self.server_dir = dir.to_string();
        create_dir_all(dir)?;
        println!("Initalizing {} at {}.", self.name, dir);
        self.initalized = true;
        let toml_config = toml::to_string(&self)?;
        write(
            format!("./.solace/servers/{}/server_config.toml", self.name),
            toml_config,
        )?;
        write(
            format!("./.solace/servers/{}/eula.txt", self.name),
            "eula=true",
        )?;

        installer::download_server(&self.server_loader, format!("{}", dir).to_string())
    }
    pub fn handle_termination(&mut self) -> Result<()> {
        let running = Arc::new(AtomicBool::new(true));

        let running_clone = running.clone();
        let config_file = format!("{}/server_config.toml", self.server_dir);

        let content = read_to_string(config_file.clone())?;
        let mut config: Server = toml::from_str(&content)?;

        config.is_running = false;

        let new_content = toml::to_string(&config)?;

        println!("{:?}", config);
        println!("{}", "Press Ctrl+C to exit.".red().bold().underline());

        let config_file_clone = config_file.clone();
        let new_content_clone = new_content.clone(); // im doing this so i can use it in ctrlc

        ctrlc::set_handler(move || {
            println!(
                "Received Ctrl+C! Performing cleanup...{}",
                "".red().bold().underline()
            );
            write(config_file_clone.clone(), new_content_clone.clone()).unwrap();

            running_clone.store(false, Ordering::SeqCst);
        })?;

        Ok(())
    }
    pub fn is_initalized(&mut self) -> bool {
        let config_file = format!("{}/server_config.toml", self.server_dir);
        println!("{}", config_file.red());

        let Ok(content) = read_to_string(config_file.clone()) else {
            eprintln!("Error reading config. This means the server is not initalized! ",);
            return false;
        };
        // if content.contains("initalized = true") {
        //     println!("Initalized true: {}", content.contains("initalized = true"));
        //     true
        // } else {
        //     println!("Not initalized");
        //     return false;
        // }
        content
            .contains("initalized = true")
            .then(|| {
                println!("Initalized true: {}", content.contains("initalized = true"));
                true
            })
            .unwrap_or_else(|| {
                println!("Not initialized");
                false
            })
    }
    pub fn exit(&mut self) -> Result<()> {
        let config_file = format!("{}/server_config.toml", self.server_dir);
        let content = read_to_string(config_file.clone())?;
        let mut config: Server = toml::from_str(&content)?;
        config.is_running = false;
        let new_content = toml::to_string(&config)?;
        println!("{:?}", config);
        write(format!("{}", config_file.clone()), new_content)?;
        Ok(())
    }
}
