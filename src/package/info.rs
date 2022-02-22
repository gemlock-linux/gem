use serde_derive::Deserialize;
use version_compare::Version;

use crate::error::*;

pub mod sections {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
    pub struct Package {
        pub name: String,
        pub version: String,
        pub description: Option<String>,
        pub license: Option<String>,
        pub license_file: Option<String>,
        pub maintainers: Option<Vec<String>>,
        pub system_lib: Option<bool>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Build {
        pub script: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Dependency {
        pub name: String,
        pub version: String,
    }
}

#[derive(Debug, Deserialize)]
pub struct TomlPackageInfo {
    pub package: sections::Package,
    pub build: sections::Build,
    pub dependencies: Option<Vec<sections::Dependency>>,
}

impl TomlPackageInfo {
    pub fn new(toml: &str) -> Result<TomlPackageInfo> {
        let pkg_info: TomlPackageInfo = match toml::from_str(toml) {
            Ok(t) => t,
            Err(e) => return Err(ErrorKind::TomlError(e)),
        };

        // Verify that the version is in the correct format.
        match Version::from(&pkg_info.package.version) {
            Some(val) => val,
            None => {
                return Err(ErrorKind::VersionError(format!(
                    "Invalid version: {}, couldn't parse",
                    pkg_info.package.version
                )))
            }
        };

        Ok(pkg_info)
    }
}
