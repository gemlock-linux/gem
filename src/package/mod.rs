use std::{fs, path::PathBuf};

use super::error::*;

mod compiler;
mod definition;
mod info;

pub use info::*;

pub struct Package {
    pub info: TomlPackageInfo,
}

impl Package {
    pub fn from_path<P: Into<PathBuf>>(path: P) -> Result<Package> {
        let path = path.into();

        // read package.toml
        let toml = match fs::read_to_string(path.join("package.toml")) {
            Ok(t) => t,
            Err(e) => return Err(ErrorKind::IoError(e)),
        };

        let info = TomlPackageInfo::new(&toml)?;

        Ok(Package { info })
    }
}
