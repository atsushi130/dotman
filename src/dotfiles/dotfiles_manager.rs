
use super::DotfilesRepository;
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

    pub fn create_url(&self, file: &str) -> String {
        match file {
            "aliasrc" => format!("{}/.aliasrc", DotfilesManager::BASE_URL),
            _ => "".to_string()
        }
    }

    pub fn create_local_path(&self, file: &str) -> String {
        match file {
            "aliasrc" => format!("~/.aliasrc"),
            _ => "".to_string()
        }
    }

    pub fn sync(&self, file: &str) {
        let url = self.create_url(file);
        DotfilesRepository.fetch(&url)
            .into_iter()
            .for_each(|content| {
                println!("{}", content);
            })
    }

    pub fn backup(&self, file: &str) {
        let path = self.create_local_path(file);
        DotfilesRepository.backup(&path);
    }
}
