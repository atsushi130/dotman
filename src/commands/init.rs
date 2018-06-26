
use super::super::clap::{ App, SubCommand, ArgMatches };
use super::super::DotfilesManager;
use std::marker::PhantomData;

pub struct Init<'a> {
    phantom: PhantomData<&'a str>
}

impl<'a> Init<'a> {

    pub const NAME: &'a str = "init";

    pub fn new<'b>() -> Init<'b> {
        Init {
            phantom: PhantomData
        }
    }

    pub fn command(&self) -> App {
        SubCommand::with_name(Init::NAME)
            .about("initialize dotfiles.")
    }

    pub fn execute(&self, _matches: &ArgMatches) {
        DotfilesManager::new().execute_init();
    }
}
