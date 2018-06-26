
// Copyright (c) 2018 Atsushi Miyake. All rights reserved.

use super::DotfilesRepository;
use super::super::{ SettingsRepository, Dotfile };
use std::marker::PhantomData;

pub struct DotfilesManager<'a> {
    phantom: PhantomData<&'a str>,
    repository: String
}

impl<'a> DotfilesManager<'a> {

    pub fn new<'b>() -> DotfilesManager<'b> {
        let settings = SettingsRepository.load();
        DotfilesManager {
            phantom: PhantomData,
            repository: format!("https://raw.githubusercontent.com/{}", settings.repository)
        }
    }

    fn create_url(&self, file: &str) -> String {
        let base_url = format!("{}/master", self.repository);
        self.find_dotfile(file)
            .map_or("".to_string(), |dotfile| format!("{}{}", base_url, dotfile.input))
    }

    fn create_local_path(&self, file: &str) -> String {
        self.find_dotfile(file)
            .map_or("".to_string(), |dotfile| dotfile.output)
    }

    pub fn find_dotfile(&self, file: &str) -> Option<Dotfile> {
        SettingsRepository.load().dotfiles
            .into_iter()
            .filter(|dotfile| dotfile.name == file.to_string())
            .next()
    }

    pub fn sync(&self, file: &str) {
        self.backup(file);
        self.create_file(file);
    }

    fn create_file(&self, file: &str) {
        let url = self.create_url(file);
        let path = self.create_local_path(file);
        DotfilesRepository.create(&url, &path);
    }

    fn backup(&self, file: &str) {
        let path = self.create_local_path(file);
        DotfilesRepository.backup(&path);
    }
}
