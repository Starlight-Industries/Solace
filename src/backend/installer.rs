use color_eyre::Result;
use colored::{self, Colorize};
use reqwest::blocking::get as download;
use serde_json;
use serde_json::Value;
use solace::{Loader, LoaderType};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn download_server(loader: &Loader, dir: String) -> Result<()> {
    let mut target: String = "".to_string();
    match loader.typ {
        LoaderType::Vanilla => {
            println!("{}", "Downloading vanilla".blue());

            target = get_vanilla_jar_url(loader.version.as_str()).unwrap();
        }
        LoaderType::Fabric => {
            println!("{}", "Downloading fabric".blue());
            target = format!(
                "https://meta.fabricmc.net/v2/versions/loader/{}/0.16.5/1.0.1/server/jar",
                loader.version
            );
        }
        LoaderType::Paper => {
            println!("{}", "Downloading Paper".blue());
            println!("{}", get_paper_jar_url(loader.version.as_str()).unwrap());
            target = get_paper_jar_url(loader.version.as_str()).unwrap();
        }
        _ => todo!(),
    }
    println!("Target is : {}", target);
    let response = download(target)?;
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

fn get_vanilla_jar_url(version: &str) -> Result<String, Box<dyn Error>> {
    let manifest_url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
    let manifest_response = reqwest::blocking::get(manifest_url)?;

    if !manifest_response.status().is_success() {
        return Err(format!(
            "Manifest download failed with status: {}",
            manifest_response.status().to_string()
        )
        .into());
    }

    let manifest_text = manifest_response.text()?;
    let manifest: Value = serde_json::from_str(&manifest_text)?;

    let version_url = manifest["versions"]
        .as_array()
        .and_then(|versions| versions.iter().find(|v| v["id"].as_str() == Some(version)))
        .and_then(|v| v["url"].as_str())
        .ok_or_else(|| format!("Version {} not found in manifest", version))?;

    let version_response = reqwest::blocking::get(version_url)?;

    if !version_response.status().is_success() {
        return Err(format!(
            "Version JSON download failed with status: {}",
            version_response.status().to_string()
        )
        .into());
    }

    let version_json: Value = version_response.json()?;

    let server_jar_path = version_json["downloads"]["server"]["url"]
        .as_str()
        .ok_or_else(|| format!("Server jar URL not found for version {}", version))?;

    println!("{}", server_jar_path.yellow());

    Ok(server_jar_path.to_string())
}

fn get_paper_jar_url(version: &str) -> Result<String, Box<dyn Error>> {
    // Define the manifest URL
    let manifest_url = "https://qing762.is-a.dev/api/papermc";

    let manifest_response = reqwest::blocking::get(manifest_url)?;

    if !manifest_response.status().is_success() {
        return Err(format!(
            "Manifest download failed with status: {}",
            manifest_response.status().to_string()
        )
        .into());
    }

    let manifest_text = manifest_response.text()?;

    let manifest: Value = serde_json::from_str(&manifest_text)?;

    let version_url = manifest["versions"]
        .as_object()
        .and_then(|versions| versions.get(version))
        .and_then(|url| url.as_str())
        .ok_or_else(|| format!("Version {} not found in manifest", version))?;

    println!("{}", version_url);

    Ok(version_url.to_string())
}
