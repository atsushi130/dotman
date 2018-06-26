
use super::super::clap::{ App, SubCommand, ArgMatches };
use super::super::SettingsRepository;
use std::marker::PhantomData;

pub struct List<'a> {
    phantom: PhantomData<&'a str>
}

impl<'a> List<'a> {

    pub const NAME: &'a str = "list";

    pub fn new<'b>() -> List<'b> {
        List {
            phantom: PhantomData
        }
    }

    pub fn command(&self) -> App {
        SubCommand::with_name(List::NAME)
            .about("show installable dotfiles.")
    }

    pub fn execute(&self, _matches: &ArgMatches) {
        SettingsRepository.load()
            .dotfiles
            .into_iter()
            .map(|dotfile| dotfile.name)
            .for_each(|dotfile_name| println!("{}", dotfile_name))
    }
}
