#![feature(const_fmt_arguments_new)]
use backend::Server;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use colored::Colorize;
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
    Gui,
}

fn start_server(server: &str) -> Result<()> {
    let mut server = Server::construct(&server.to_lowercase(), 25565, LoaderType::None, "1.21.1")?;
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
        Some(Commands::Gui) => Ok(app::main().unwrap()),
        None => Ok({
            let no_command_message = format!(
                "{}",
                "No command was specified! Run 'solace --help' for more info!"
                    .red()
                    .bold()
            );
            println!("{}", no_command_message);
            println!("{}", backend::metadata::get_working_dir());
            println!("{}", backend::metadata::get_server_dir());
            println!(
                "{}",
                "Solace will be a gui app in the future! Assuming you wanted the gui.."
                    .green()
                    .underline()
            );
            app::main().unwrap();
        }),
    }
}
