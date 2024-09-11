use super::Loader;
use colored::{self, Colorize};
use std::io::{BufWriter, Write};
use std::fs::File;
use std::string;

pub fn download_server(loader: &Loader,dir: String ) -> Result<(), Box<dyn std::error::Error>> {
    let target: String;
    match loader.typ {
        super::LoaderType::Vanilla => {
            println!("{}","Downloading vanilla".blue());

            target = format!("https://launcher.mojang.com/v1/objects/{}/server.jar",loader.version);
        }
        super::LoaderType::Fabric => {
            println!("{}","Downloading fabric".blue());
            target = format!("https://meta.fabricmc.net/v2/versions/loader/{}/0.16.5/1.0.1/server/jar",loader.version);
        }
        _ => todo!()
    }
    let response = reqwest::blocking::get(&target)?;
    if response.status().is_success() {
        let file = File::create(format!("{}/server.jar", dir))?;

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
    Ok(())
}
fn parse_vanilla_manifest(version: String) -> String {
    let response = reqwest::blocking::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .expect("");
    if response.status().is_success() {
        let version_manifest = response.text().expect("Failed to get response text");
        println!("{}", version_manifest);
        return version_manifest;
    } else {
        eprintln!("Download failed with status: {}", response.status().to_string());
        return "asdflkjasdhlf".to_string();
    }
}