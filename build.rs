// SPDX-License-Identifier: MIT

// Copyright (C) 2020 Malinovsky Rodion (rodionmalino@gmail.com)

use std::path::Path;
use std::{env, fs};

const CONFIG_FILE_NAME: &str = "config.toml";

fn main() -> std::io::Result<()> {
    let target_dir_path = env::var("OUT_DIR").unwrap();
    let dest_config_path = Path::new(&target_dir_path)
        .join("../../..")
        .join(CONFIG_FILE_NAME);
    if !dest_config_path.exists() {
        print!("Copy default config");
        fs::copy(CONFIG_FILE_NAME, dest_config_path)?;
    } else {
        print!("Skip copy default config. Already exists.");
    }
    Ok(())
}
