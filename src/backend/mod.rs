use std::{error::Error, fs, process::{Command, Stdio}, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use colored::Colorize;
use serde::{Deserialize, Serialize};
mod installer;
#[derive(Serialize,Deserialize,Debug)]
struct Loader {
    typ: LoaderType,
    version: String,
    // common data here
}
#[derive(Serialize,Deserialize,Debug)]
enum LoaderType {
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

#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct Server {
    name: String,
    port: u16,
    server_dir: String,
    server_loader: Loader,
    is_running: bool,
    initalized: bool,
}

impl Server {
    pub fn construct(name: &str, port: u16) -> Self {
        println!("Constructing server {} with port {}", name.blue(), port.to_string().green());
        
        let config_path = format!("./.solace/servers/{}/server_config.toml", name);
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(server) = toml::from_str::<Server>(&content) {
                if server.name == name {
                    println!("Loading existing server configuration for {}", name.blue());
                    return server;
                }
            }
        }

        return Self {
            name: name.to_string(),
            port,
            server_dir: format!("./.solace/servers/{}", name),
            server_loader: Loader {
                typ: LoaderType::Fabric,
                version: "1.21.1".to_string()
            },
            is_running: false,
            initalized: false,
        };
    }
    pub fn start_server(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.is_running && self.initalized {
            println!("starting {} on {}", self.name.blue(),self.port.to_string().green());
            
            let file_path = format!("./server.jar");
            let config_file: String = format!("{}/server_config.toml",self.server_dir);
            println!("{}",config_file);
            let content: String = fs::read_to_string(config_file.clone()).expect("failed to read toml");
            let mut config: Server = toml::from_str(&content).expect("could not read config when starting the server");
            config.is_running = true;
            let new_content = toml::to_string(&config).expect("could not generate new content");
            println!("{:?}",config);
            fs::write(format!("{}", config_file.clone()),new_content).expect("failed to mark server as online");
            let _ = self.handle_termination();
            
            let status = Command::new("java")
            .arg("-jar")
            .arg(file_path)
            //.arg("-nogui")
            .current_dir(self.server_dir.clone())
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status().expect("Failed to launch server");

            Ok(())
        } else if !self.is_running && !self.initalized {
            println!("Server not initalized. Finishing process");
            self.init();
            Ok(())
        } else {
            println!("{} server: {} already started","Err:".red(),self.name);
            Ok(())
        }
    }

    pub fn init(&mut self)  {
        //let current_path = env::current_dir();
        let dir: &str = &format!("./.solace/servers/{}",& mut self.name);
        self.server_dir = dir.to_string();
        let _ = fs::create_dir_all(dir);
        println!("Initalizing {} at {}.",self.name,dir);
        self.initalized = true;
        let toml_config = toml::to_string(&self).unwrap();
        fs::write(format!("./.solace/servers/{}/server_config.toml",self.name), toml_config);
        fs::write(format!("./.solace/servers/{}/eula.txt",self.name), "eula=true");
        
        installer::download_server(&self.server_loader, format!("{}",dir).to_string());
    }
    pub fn handle_termination(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let running = Arc::new(AtomicBool::new(true));
    
        let running_clone = running.clone();
        let config_file = format!("{}/server_config.toml", self.server_dir);
        
        let content = fs::read_to_string(config_file.clone())?;
        let mut config: Server = toml::from_str(&content)?;
        
        config.is_running = false;

        let new_content = toml::to_string(&config)?;
    
        println!("{:?}", config);
        println!("{}", "Press Ctrl+C to exit.".red().bold().underline());
    
        let config_file_clone = config_file.clone();
        let new_content_clone = new_content.clone(); // im doing this so i can use it in ctrlc
    
        ctrlc::set_handler(move || {
            println!("Received Ctrl+C! Performing cleanup...");
            if let Err(e) = fs::write(config_file_clone.clone(), new_content_clone.clone()) {
                eprintln!("Failed to write config file: {}", e);
            }
    
            running_clone.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl+C handler");
    
        Ok(())
    }
    pub fn is_initalized(&mut self) -> bool {
        let config_file = format!("{}/server_config.toml", self.server_dir);
        println!("{}", config_file.red());
        
        let content = match fs::read_to_string(config_file.clone()) {
            Ok(data) => {
                println!("Succsessfully read file");
                data
            }
            Err(e) => {
                eprintln!("Error reading config: {} This means the server is not initalized! ",e);
                return false;
            }
        };
        let config: Server = toml::from_str(&content).expect("failed to read config to check initaliziaon");
        if content.contains("initalized = true") {
            println!("Initalized true: {}", content.contains("initalized = true"));
            return true;
        } else {
            println!("Not initalized");
            return false;
        }
    }
}