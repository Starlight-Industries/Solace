use std::fmt::format;

use backend::Server;
use serde::ser;
use versions::Versioning;
use clap::{Parser, Subcommand};


pub mod solace_app;
pub mod backend;
const SERVER_DIR: &str = "./.solace/servers/";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// Runs the server with the name you specify
    Run {
        /// Name of the server to run
        #[arg(short, long)]
        server: String,
    },
    /// Creates a server and initalizes it
    Create {
    }
}
fn start_server(server: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::construct(&server.to_lowercase(), 25565);
    if server.is_initalized() != true {
        server.init()
    } else {
        //std::process::exit(0)
    }
    let _ = server.start_server();
    Ok(())
}
fn main() {
    // let mut test_server = backend::Server::construct("TestServer", 25565);
    // if test_server.is_initalized() != true {
    //     test_server.init()
    // } else {
    //     std::process::exit(0)
    // }
    // println!("{}", toml);
    // let _ = test_server.start_server();

    //launch(solace_app::_app);
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run {server }) => {
            start_server(server).expect("failed to start server");
        }
        Some(Commands::Create {  }) => {
            Server::create_server();
        }
        None => {}
    }
}