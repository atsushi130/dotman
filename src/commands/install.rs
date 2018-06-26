
// Copyright (c) 2018 Atsushi Miyake. All rights reserved.

use super::super::clap::{ App, Arg, SubCommand, ArgMatches };
use super::super::DotfilesManager;
use std::marker::PhantomData;

pub struct Install<'a> {
    phantom: PhantomData<&'a str>
}

impl<'a> Install<'a> {

    pub const NAME: &'a str = "install";

    pub fn new<'b>() -> Install<'b> {
        Install {
            phantom: PhantomData
        }
    }

    pub fn command(&self) -> App {
        SubCommand::with_name(Install::NAME)
            .about("Install specified dotfiles from your repository.")
            .arg(
                Arg::with_name("rc").multiple(true)
            )
    }

    pub fn execute(&self, matches: &ArgMatches) {
        if let Some(dotfile) = matches.values_of("rc") {
            dotfile
                .into_iter()
                .for_each(|dotfile| {
                    DotfilesManager::new().sync(dotfile);
                    self.chain(dotfile);
                })
        } else {
            println!("install command required arguments.");
        }
    }

    fn chain(&self, dotfile: &str) {
        let manager = DotfilesManager::new();
        manager.find_dotfile(dotfile)
            .into_iter()
            .flat_map(|dotfile| dotfile.chain)
            .next()
            .into_iter()
            .flat_map(|chain| chain)
            .for_each(|dotfile| manager.sync(&dotfile))
    }
}