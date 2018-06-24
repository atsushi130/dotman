
// Copyright (c) 2018 Atsushi Miyake. All rights reserved.

use super::super::clap::{ App, Arg, SubCommand, ArgMatches };
use super::super::DotfilesManager;
use std::marker::PhantomData;

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
                DotfilesManager::new().sync(rc);
            })
        } else {
            println!("install command required arguments.");
        }
    }
}