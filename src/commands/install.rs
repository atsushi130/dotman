

use super::super::clap::{ App, Arg, SubCommand };
pub struct Install;

impl Install {

    pub fn command(&self) -> App {
        SubCommand::with_name("install")
            .about("Install specified dotfiles from your repository.")
            .arg(
                Arg::with_name("rc").multiple(true)
            )
    }
}