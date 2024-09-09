use super::Loader;
use colored::{self, Colorize};
use freya::events::file;
use std::io::{self, copy, BufWriter, Cursor, Write};
use std::fs::File;

pub fn download_server(loader: &Loader,dir: String ) -> Result<(), Box<dyn std::error::Error>> {
    let target: String;
    match loader.typ {
        super::LoaderType::Fabric => {
            println!("{}","Downloading fabric".blue());
            target = format!("https://meta.fabricmc.net/v2/versions/loader/{}/0.16.5/1.0.1/server/jar",loader.version);
        }
        _ => todo!()
    }
    let response = reqwest::blocking::get(&target)?;
    if response.status().is_success() {
        let mut file = File::create(format!("{}/server.jar", dir))?;

        let content = response.bytes()?;
        if content.is_empty() {
            eprintln!("Downloaded content is empty.");
            return Err("Downloaded content is empty.".into());
        }
        let mut writer = BufWriter::new(file);
        writer.write_all(&content);
        writer.flush();
    } else {
        eprintln!("Download failed with status: {}", response.status());
    }
    //let mut out = File::create("./server.jar").expect("file creation failed");
    //io::copy(&mut output.as_bytes(), &mut out).expect("Failed to write file");
    //println!("{}", target.purple());
    Ok(())
}