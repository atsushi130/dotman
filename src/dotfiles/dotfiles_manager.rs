
use std::marker::PhantomData;

pub struct DotfilesManager<'a> {
    phantom: PhantomData<&'a str>
}

impl<'a> DotfilesManager<'a> {

    const BASE_URL: &'a str = "https://raw.githubusercontent.com/atsushi130/dotfiles/master";

    pub fn new<'b>() -> DotfilesManager<'b> {
        DotfilesManager {
            phantom: PhantomData
        }
    }

    pub fn create(&self, file: &str) -> String {
        match file {
            "aliasrc" => format!("{}/.aliasrc", DotfilesManager::BASE_URL),
            _ => "".to_string()
        }
    }
}
