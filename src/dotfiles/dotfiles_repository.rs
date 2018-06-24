
use super::DotfilesManager;
use std::process::Command;
use std::str::from_utf8;

pub struct DotfilesRepository;
impl DotfilesRepository {

    pub fn fetch(&self, file: &str) -> Option<String> {

        let dotfiles_manager = DotfilesManager::new();
        let url = dotfiles_manager.create(file);
        let command = format!("curl {}", url);
        let data = Command::new("bash")
            .arg("-c")
            .arg(command)
            .output()
            .expect("fetch failed").stdout;

        from_utf8(&data)
            .ok()
            .map(|content| format!("{}", content.to_string()))
    }
}
