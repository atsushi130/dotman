
// Copyright (c) 2018 Atsushi Miyake. All rights reserved.

use super::super::clap::{ App, SubCommand, ArgMatches };
use super::super::DotfilesService;
use std::marker::PhantomData;

pub struct Setup<'a> {
    phantom: PhantomData<&'a str>
}

impl<'a> Setup<'a> {

    pub const NAME: &'a str = "init";

    pub fn new<'b>() -> Setup<'b> {
        Setup {
            phantom: PhantomData
        }
    }

    pub fn command(&self) -> App {
        SubCommand::with_name(Setup::NAME)
            .about("setup dotfiles.")
    }

    pub fn execute(&self, _matches: &ArgMatches) {
        DotfilesService::new().execute_init();
    }
}
