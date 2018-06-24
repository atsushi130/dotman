
use std::process::Command;
use std::str::from_utf8;

pub struct DotfilesRepository;
impl DotfilesRepository {

    pub fn fetch(&self, url: &str) -> Option<String> {

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
