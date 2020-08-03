// SPDX-License-Identifier: MIT

// Copyright (C) 2020 Malinovsky Rodion (rodionmalino@gmail.com)

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub credentials: Credentials,
    pub options: Options,
}

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub server: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Options {
    pub weeks: u32,
    pub users: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Config> {
        use std::path::Path;

        let config_path =
            Path::new(&std::env::current_exe()?.parent().unwrap()).join("config.toml");
        log::debug!("config_path: {:?}", config_path);
        let config_string = std::fs::read_to_string(&config_path)?;
        log::debug!("config_string: {}", config_string);
        let config: Config = toml::from_str(&config_string)?;
        Ok(config)
    }
}
