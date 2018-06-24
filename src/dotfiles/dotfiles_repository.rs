
// Copyright (c) 2018 Atsushi Miyake. All rights reserved.

use std::process::Command;
use std::str::from_utf8;

pub struct DotfilesRepository;
impl DotfilesRepository {

    #[allow(unused)]
    pub fn fetch(&self, url: &str) -> Option<String> {

        let command = format!("curl {}", url);
        let data = self.execute_command(&command, "fetch failed.");

        from_utf8(&data)
            .ok()
            .map(|content| format!("{}", content.to_string()))
    }

    pub fn create(&self, url: &str, path: &str) {
        let command = format!("curl {} >| {}", url, path);
        self.execute_command(&command, "create failed.");
    }

    pub fn backup(&self, path: &str) {
        let command = format!("cp -f {} {}.backup", path, path);
        self.execute_command(&command, "backup failed.");
    }

    pub fn execute_command(&self, command: &str, error_message: &str) -> Vec<u8> {
        Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .expect(error_message)
            .stdout
    }
}
