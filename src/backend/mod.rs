use colored::Colorize;
use metadata::get_working_dir;
use serde::{Deserialize, Serialize};
pub mod installer;
use color_eyre::Result;
use colored::control;
use inquire::{Select, Text};
use solace::{Loader, LoaderType};
use spinoff::{spinners, Color, Spinner};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Once;
use std::thread::sleep;
use std::time::Duration;

use std::fs::{create_dir_all, read_to_string, write};
use std::process::Command;
pub mod metadata;
static CTRL_C_HANDLER: Once = Once::new();
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
    pub fn find_server(name: &str) -> Result<Self> {
        let config_path = format!(
            "{}/.solace/servers/{}/server_config.toml",
            get_working_dir(),
            name
        );
        match read_to_string(&config_path) {
            Ok(content) => {
                println!("Loading existing configuration for {}", name.blue());
                let server: Server = toml::from_str(&content)?;
                if server.name == name {
                    return Ok(server);
                } else {
                    panic!()
                }
            }
            Err(_) => {
                println!("No existing configuration found for {}", name.blue());
                panic!()
            }
        }
    }
    pub fn construct(name: &str, port: u16, loader: Loader) -> Result<Self> {
        println!(
            "Constructing server {} with port {}",
            name.blue(),
            port.to_string().green()
        );

        let config_path = format!(
            "{}/.solace/servers/{}/server_config.toml",
            get_working_dir(),
            name
        );
        match read_to_string(&config_path) {
            Ok(content) => {
                println!("Loading existing configuration for {}", name.blue());
                let server: Server = toml::from_str(&content)?;
                if server.name == name {
                    return Ok(server);
                }
            }
            Err(_) => {
                println!("No existing configuration found for {}", name.blue());
            }
        }
        Ok(Self {
            name: name.to_string(),
            port,
            server_dir: format!("{}/.solace/servers/{}", get_working_dir(), name),
            server_loader: Loader {
                typ: loader.typ,
                version: loader.version,
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

        let server_version = Text::new("Server version (ex: 1.21.1)")
            .with_default("latest")
            .prompt();

        let port = Text::new("Port: ").with_default("25565").prompt()?;

        let port_number: u16 = port.parse()?;
        let loader_instance = Loader {
            typ: loader_type,
            version: server_version.unwrap(),
        };

        let mut new_server = Server::construct(&name, port_number, loader_instance)?;

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
        let dir = &format!("{}/.solace/servers/{}", get_working_dir(), &mut self.name);
        self.server_dir = dir.to_string();
        create_dir_all(dir)?;
        println!("Initalizing {} at {}.", self.name, dir);
        self.initalized = true;
        let toml_config = toml::to_string(&self)?;
        write(
            format!(
                "{}/.solace/servers/{}/server_config.toml",
                get_working_dir(),
                self.name
            ),
            toml_config,
        )?;
        write(
            format!(
                "{}/.solace/servers/{}/eula.txt",
                get_working_dir(),
                self.name
            ),
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
        let mut terminating: bool = false;

        CTRL_C_HANDLER.call_once(|| {
            // Register the Ctrl+C handler
            ctrlc::set_handler(move || {
                if !terminating {
                    terminating = true;
                    #[cfg(target_os = "windows")]
                    {
                        control::set_virtual_terminal(true).unwrap();
                    }
                    println!(
                        "{}",
                        "Received Ctrl+C! Performing cleanup..."
                            .red()
                            .bold()
                            .underline()
                    );
                    let _spinner = Spinner::new(spinners::Arc, " Terminating... ", Color::Red);
                    write(config_file_clone.clone(), new_content_clone.clone()).unwrap();
                    running_clone.store(false, Ordering::SeqCst);
                }
            })
            .unwrap();
            println!("Attempting to terminate");
        });

        Ok(())
    }
    pub fn is_initalized(&mut self) -> bool {
        let config_file = format!("{}/server_config.toml", self.server_dir);
        println!("{}", config_file.red());

        let Ok(content) = read_to_string(config_file.clone()) else {
            eprintln!("Error reading config. This means the server is not initalized! ",);
            return false;
        };
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
