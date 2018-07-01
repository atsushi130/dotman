
// Copyright (c) 2018 Atsushi Miyake. All rights reserved.

extern crate clap;
extern crate rustc_serialize;

use clap::App;

mod commands;
use commands::{ Install, List, Setup };

mod dotfiles;
use dotfiles::DotfilesManager;

mod settings;
use settings::{ SettingsRepository, Dotfile };

fn main() {

    let dotman = App::new("dotman");

    let install = Install::new();
    let install_command = install.command();

    let list = List::new();
    let list_command = list.command();

    let setup = Setup::new();
    let setup_command = setup.command();

    let matches = dotman
        .subcommand(install_command)
        .subcommand(list_command)
        .subcommand(setup_command)
        .get_matches();

    match matches.subcommand() {
        (Install::NAME, Some(install_matches)) => {
            install.execute(install_matches);
        },
        (List::NAME, Some(list_matches)) => {
            list.execute(list_matches)
        },
        (Setup::NAME, Some(setup_matches)) => {
            setup.execute(setup_matches)
        }
        _  => {}
    }
}
