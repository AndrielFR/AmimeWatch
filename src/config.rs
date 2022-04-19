// SPDX-License-Identifier: MIT You, 2022-03-09 - initial commit
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use std::fs::File;
use std::io::{Read, Write};

use serde_derive::{Deserialize, Serialize};

const PATH: &str = "./config.toml";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub telegram: Telegram,
    pub bot: Bot,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut toml_str = String::new();
        File::open(PATH).and_then(|mut file| file.read_to_string(&mut toml_str))?;

        Ok(toml::from_str::<Self>(&toml_str)?)
    }

    pub fn save(&self) -> Result<()> {
        let toml = toml::to_string_pretty(&self)?;
        File::open(PATH).and_then(|mut file| file.write(toml.as_bytes()))?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Telegram {
    pub api_id: i32,
    pub api_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bot {
    pub token: String,
    pub prefixes: Vec<String>,
}
