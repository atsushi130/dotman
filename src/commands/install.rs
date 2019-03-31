
// Copyright (c) 2018 Atsushi Miyake. All rights reserved.

use super::super::clap::{ App, Arg, SubCommand, ArgMatches };
use super::super::DotfilesService;
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
            .arg(
                Arg::with_name("chain")
                    .help("chain install")
                    .short("c")
                    .long("chain")
            )
    }

    pub fn execute(&self, matches: &ArgMatches) {
        if let Some(mut values) = matches.values_of("rc") {
            if let Some(dotfile) = values.next() {
                self.install(dotfile, matches.is_present("chain"));
            }
        } else {
            println!("install command required arguments.");
        }
    }

    fn install(&self, dotfile: &str, is_chain: bool) {
        DotfilesService::new().sync(dotfile);
        if is_chain {
            self.chain(dotfile);
        }
    }

    fn chain(&self, dotfile: &str) {
        let service = DotfilesService::new();
        service.find_dotfile(dotfile)
            .into_iter()
            .flat_map(|dotfile| dotfile.chain)
            .next()
            .into_iter()
            .flat_map(|chain| chain)
            .for_each(|dotfile| service.sync(&dotfile))
    }
}