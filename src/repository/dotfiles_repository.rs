
use std::process::{ Command, Stdio };
use std::str::from_utf8;

pub struct DotfilesRepository;
impl DotfilesRepository {

    pub fn fetch(&self, file: &str) -> Option<String> {

        let url_factory = DotfilesUrlFactory::new();
        let url = url_factory.create(file);
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

use std::marker::PhantomData;

struct DotfilesUrlFactory<'a> {
    phantom: PhantomData<&'a str>
}

impl<'a> DotfilesUrlFactory<'a> {

    const BASE_URL: &'a str = "https://raw.githubusercontent.com/atsushi130/dotfiles/master";

    pub fn new<'b>() -> DotfilesUrlFactory<'b> {
        DotfilesUrlFactory {
            phantom: PhantomData
        }
    }

    pub fn create(&self, file: &str) -> String {
        match file {
            "aliasrc" => format!("{}/.aliasrc", DotfilesUrlFactory::BASE_URL),
            _ => "".to_string()
        }
    }
}