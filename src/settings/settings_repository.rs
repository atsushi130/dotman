
// Copyright (c) 2018 Atsushi Miyake. All rights reserved.

use super::Settings;
use rustc_serialize::json;
use std::fs::File;
use std::io::Read;

pub struct SettingsRepository;

impl SettingsRepository {

    pub fn load(&self) -> Settings {

        let dir = env!("CARGO_MANIFEST_DIR");
        let path = format!("{}/settings.json", dir);
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let settings: Settings = json::decode(&data).unwrap();
        settings
    }
}