use serde_derive::{Deserialize, Serialize};

use super::{sections, TomlPackageInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct PKGFile {
    pub dst: String,
    pub mode: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDefinition {
    pub name: String,                            //
    pub version: String,                         //
    pub description: Option<String>,             //
    pub license: Option<String>,                 //
    pub maintainers: Option<Vec<String>>,        //
    pub files: Vec<PKGFile>,                     // Files to install / uninstall
    pub dependencies: Vec<sections::Dependency>, // Dependencies
}

impl PackageDefinition {
    pub fn new(info: &TomlPackageInfo) -> PackageDefinition {
        let dependencies = info
            .dependencies
            .as_ref()
            .map(|deps| deps.clone())
            .unwrap_or_else(|| Vec::new());

        PackageDefinition {
            name: info.package.name.clone(),
            version: info.package.version.clone(),
            description: info.package.description.clone(),
            license: info.package.license.clone(),
            maintainers: info.package.maintainers.clone(),
            files: Vec::new(),
            dependencies,
        }
    }

    pub fn add_file(&mut self, dst: String, mode: u32) {
        self.files.push(PKGFile { dst, mode });
    }
}
