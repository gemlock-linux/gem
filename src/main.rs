use clap::Command;

use crate::modules::*;

mod error;
mod modules;
mod package;

#[tokio::main]
async fn main() {
    let modules = [BuildModule::default()];

    let clap = Command::new("Gem")
        .version("v0.1.0")
        .author("Robin Alexander Plate")
        .about("Gemlock/Linux package manager")
        .subcommands(
            modules
                .iter()
                .map(|module| module.command())
                .collect::<Vec<_>>(),
        )
        .arg_required_else_help(true)
        .get_matches();

    modules
        .iter()
        .find(|module| module.command().get_name() == clap.subcommand().unwrap().0)
        .unwrap()
        .entry(clap.subcommand().unwrap().1)
        .await;
}
