use color_eyre::eyre::bail;
use color_eyre::Result;
use colored::{self, Colorize};
use reqwest::blocking::get as download;
use solace::{Loader, LoaderType};
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn download_server(loader: &Loader, dir: String) -> Result<()> {
    let target: String;
    match loader.typ {
        LoaderType::Vanilla => {
            println!("{}", "Downloading vanilla".blue());

            target = format!(
                "https://launcher.mojang.com/v1/objects/{}/server.jar",
                loader.version
            );
        }
        LoaderType::Fabric => {
            println!("{}", "Downloading fabric".blue());
            target = format!(
                "https://meta.fabricmc.net/v2/versions/loader/{}/0.16.5/1.0.1/server/jar",
                loader.version
            );
        }
        _ => todo!(),
    }
    let response = download(&target)?;
    if response.status().is_success() {
        let file = File::create(format!("{}/server.jar", dir))?;

        let content = response.bytes()?;
        if content.is_empty() {
            eprintln!("Downloaded content is empty.");
            color_eyre::eyre::bail!("Downloaded content is empty.");
        }
        let mut writer = BufWriter::new(file);
        writer.write_all(&content)?;
        writer.flush()?;
    } else {
        eprintln!("Download failed with status: {}", response.status());
    }
    Ok(())
}
#[allow(unused)]
fn parse_vanilla_manifest(version: String) -> Result<String> {
    let response =
        reqwest::blocking::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
            .expect("");
    if response.status().is_success() {
        let version_manifest = response.text()?;
        println!("{}", version_manifest);
        return Ok(version_manifest);
    } else {
        bail!(
            "Download failed with status: {}",
            response.status().to_string()
        );
    }
}
