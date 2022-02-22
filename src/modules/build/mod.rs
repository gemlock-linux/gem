use std::env;

use async_trait::async_trait;

use crate::package::Package;

use super::Module;

#[derive(Default)]
pub struct BuildModule;

#[async_trait]
impl Module for BuildModule {
    fn command(&self) -> clap::Command {
        clap::Command::new("build")
            .about("Builds a package") //
            .arg(
                clap::Arg::new("directory")
                    .help("Input directory of the package")
                    .takes_value(true)
                    .default_value("."),
            )
    }

    async fn entry(&self, clap: &clap::ArgMatches) {
        let directory = clap.value_of("directory").unwrap();

        println!("Building package in {}", directory);
        let package = Package::from_path(directory).unwrap();

        // change the current working directory to the input directory
        // otherwise we might end up building in the wrong directory :c
        env::set_current_dir(directory).unwrap();

        package.build().await.unwrap();
    }
}
