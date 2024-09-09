use std::{env, error::Error, fs, process::{Command, Stdio}};

use colored::Colorize;
use serde::Serialize;
mod installer;
#[derive(serde::Serialize)]
struct Loader {
    typ: LoaderType,
    version: String,
    // common data here
}
#[derive(serde::Serialize)]
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

#[derive(Serialize)]
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
        println!("Constructing server {} with port {}",name.blue(), port.to_string().green(),);
        return Self {
            name: name.to_string(),
            port,
            server_dir: "".to_owned(),
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
            let status = Command::new("java")
            .arg("-jar")
            .arg(file_path)
            .arg("-nogui")
            .current_dir(self.server_dir.clone())
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;
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
        
        installer::download_server(&self.server_loader, format!("{}",dir).to_owned());
    }
    pub fn is_initalized(&mut self) -> bool {
        if self.initalized {
            return true;
        } else {
            return false;
        }
    }
}