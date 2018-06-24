
extern crate clap;
use clap::{ App, Arg, SubCommand };

fn main() {

    let dotman = App::new("dotman");

    let install = "install";
    let install_command = SubCommand::with_name(install)
        .about("Install specified dotfiles from your repository.")
        .arg(
            Arg::with_name("rc").multiple(true)
        );

}
