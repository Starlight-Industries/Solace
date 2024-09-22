use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Loader {
    pub typ: LoaderType,
    pub version: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LoaderType {
    Vanilla,
    Forge,
    Fabric,
    Quilt,
    Paper,
    Spigot,
    Bukkit,
    Purpur,
    Sponge,
    Bungee,
    Velocity,
    Folia,  // nobody is gonna use folia lol
    Mohist, // Nobody is gonna use mohist either
    None,
}

impl Display for LoaderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use LoaderType::*;
        write!(
            f,
            "{}",
            match self {
                Forge => "Forge",
                Fabric => "Fabric",
                Quilt => "Quilt",
                Paper => "Paper",
                Spigot => "Spigot",
                Bukkit => "Bukkit",
                Purpur => "Purpur",
                Sponge => "Sponge",
                Bungee => "Bungee",
                Velocity => "Velocity",
                Folia => "Folia",
                Mohist => "Mohist",
                _ => "Vanilla",
            }
        )
    }
}

pub struct Config {
    backup_dest: Path,
}
