
extern crate clap;
use clap::App;

use std::process::{ Command, Stdio };

mod commands;
use commands::Install;

fn main() {

    let dotman = App::new("dotman");

    let install = Install::from();
    let install_command = install.command();
    let matches = dotman.subcommand(install_command).get_matches();
    match matches.subcommand() {
        (Install::NAME, Some(install_matches)) => {
            install.execute(install_matches);
        },
        _  => {}
    }
}
