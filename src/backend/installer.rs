use color_eyre::eyre::bail;
use color_eyre::Result;
use colored::{self, Colorize};
use reqwest::blocking::get as download;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::from_str;
use serde_json::Value;
use solace::{Loader, LoaderType};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Deserialize, Serialize)]
struct MinecraftVersionManifest {
    latest: String,
    versions: Vec<MinecraftVersion>,
}

#[derive(Deserialize, Serialize)]
struct MinecraftVersion {
    id: String,
    url: String,
    time: String,
    release_time: String,
    downloads: MinecraftVersionDownloads,
}

#[derive(Deserialize, Serialize)]
struct MinecraftVersionDownloads {
    client: String,
    server: String,
}

pub fn download_server(loader: &Loader, dir: String) -> Result<()> {
    let mut target: String = "".to_string();
    match loader.typ {
        LoaderType::Vanilla => {
            println!("{}", "Downloading vanilla".blue());
            target = get_server_jar_url(loader.version.as_str()).unwrap();
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

fn get_server_jar_url(version: &str) -> Result<String, Box<dyn Error>> {
    // First, fetch the version manifest
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

    // Extract the server jar URL
    let server_jar_path = version_json["downloads"]["server"]["url"]
        .as_str()
        .ok_or_else(|| format!("Server jar URL not found for version {}", version))?;

    // Construct the full URL
    println!("{}", server_jar_path.yellow());
    let full_url = Url::parse("https://launcher.mojang.com")?.join(server_jar_path)?;

    Ok(server_jar_path.to_string())
}
