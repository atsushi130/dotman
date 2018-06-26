
// Copyright (c) 2018 Atsushi Miyake. All rights reserved.

extern crate clap;
extern crate rustc_serialize;

use clap::App;

mod commands;
use commands::{ Install, List, Init };

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

    let init = Init::new();
    let init_command = init.command();

    let matches = dotman
        .subcommand(install_command)
        .subcommand(list_command)
        .subcommand(init_command)
        .get_matches();

    match matches.subcommand() {
        (Install::NAME, Some(install_matches)) => {
            install.execute(install_matches);
        },
        (List::NAME, Some(list_matches)) => {
            list.execute(list_matches)
        },
        (Init::NAME, Some(init_matches)) => {
            init.execute(init_matches)
        }
        _  => {}
    }
}
