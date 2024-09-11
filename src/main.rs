use backend::Server;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use solace::LoaderType;

pub mod app;
pub mod backend;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long)]
        name: String,
    },
    Create,
}

fn start_server(server: &str) -> Result<()> {
    let mut server = Server::construct(&server.to_lowercase(), 25565, LoaderType::None)?;
    if !server.is_initalized() {
        server.init()?;
    }
    server.start_server()
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { name }) => start_server(name),
        Some(Commands::Create) => Server::create_server(),
        None => Ok(()),
    }
}
