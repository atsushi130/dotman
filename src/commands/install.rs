

use super::super::clap::{ App, Arg, SubCommand, ArgMatches };
use std::process::{ Command, Stdio };
use std::str::from_utf8;

pub struct Install;

impl Install {

    pub fn command(&self) -> App {
        SubCommand::with_name("install")
            .about("Install specified dotfiles from your repository.")
            .arg(
                Arg::with_name("rc").multiple(true)
            )
    }

    pub fn execute(&self, matches: &ArgMatches) {
        if let Some(rcs) = matches.values_of("rc") {
                rcs.into_iter().for_each(|rc| {
                    let a = Command::new("bash")
                        .arg("-c")
                        .arg(format!("echo {}", rc.to_string()))
                        .output()
                        .expect("install failed").stdout;
                    print!("{}", from_utf8(&a).unwrap());
                })
            } else {
                println!("install command required arguments.");
            }
    }
}