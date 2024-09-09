use super::Loader;
use colored::{self, Colorize};

pub fn download_server(loader: &Loader,dir: String ) {
    match loader.typ {
        super::LoaderType::Fabric => {
            println!("{}","Downloading fabric".blue())
        }
        _ => todo!()
    }
}