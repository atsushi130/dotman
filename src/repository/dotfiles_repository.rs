
use std::process::{ Command, Stdio };
use std::str::from_utf8;

pub struct DotfilesRepository;

impl DotfilesRepository {

    pub fn fetch(&self, file: &str) {
        let url = format!("https://raw.githubusercontent.com/atsushi130/dotfiles/master/.{}", file);
        let command = format!("curl {}", url);
        let a = Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .expect("fetch failed").stdout;
    }
}