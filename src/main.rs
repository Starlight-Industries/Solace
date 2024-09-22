use backend::Server;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use colored::Colorize;
use solace::{Loader, LoaderType};

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

fn start_server(mut server: Server) -> Result<()> {
    if !server.is_initalized() {
        server.init()?;
    }
    server.start_server()
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { name }) => {
            let mut server_to_run = Server::find_server(name).unwrap();
            if Server::is_initalized(&mut server_to_run) {
                start_server(server_to_run)
            } else {
                panic!()
            }
        }
        Some(Commands::Create) => Server::create_server(),
        Some(Commands::Gui) => Ok(app::main().unwrap()),
        None => Ok({
            backend::metadata::get_server_list();
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
