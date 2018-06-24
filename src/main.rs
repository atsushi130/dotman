
extern crate clap;
use clap::{ App, Arg, SubCommand };

use std::process::{ Command, Stdio };

fn main() {

    let dotman = App::new("dotman");

    let install = "install";
    let install_command = SubCommand::with_name(install)
        .about("Install specified dotfiles from your repository.")
        .arg(
            Arg::with_name("rc").multiple(true)
        );

    let matches = dotman.subcommand(install_command).get_matches();
    match matches.subcommand() {
        (_install, Some(install_matches)) => {
            if let Some(rcs) = install_matches.values_of("rc") {
                rcs.into_iter().for_each(|rc| {
                    print!("{}", rc.to_string());
                    let a = Command::new("bash")
                        .arg("-c")
                        .arg(format!("echo aaa{}", rc.to_string()))
                        .output()
                        .expect("aaaa").stdout;
                    print!("{}", std::str::from_utf8(&a).unwrap());
                })
            } else {
                println!("install command required arguments.");
            }
        },
        _  => {}
    }
}
