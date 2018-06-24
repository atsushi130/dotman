

use super::super::clap::{ App, Arg, SubCommand, ArgMatches };
use super::super::DotfilesRepository;
use std::process::{ Command, Stdio };
use std::marker::PhantomData;
use std::str::from_utf8;

pub struct Install<'a> {
    phantom: PhantomData<&'a str>
}

impl<'a> Install<'a> {

    pub const NAME: &'a str = "install";

    pub fn from<'b>() -> Install<'b> {
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
        if let Some(rcs) = matches.values_of("rc") {
            rcs.into_iter().for_each(|rc| {
                DotfilesRepository.fetch(rc)
                    .into_iter()
                    .for_each(|content| {
                        println!("{}", content);
                    })
            })
        } else {
            println!("install command required arguments.");
        }
    }
}