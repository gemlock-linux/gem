use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::process;
use std::{fs, path::PathBuf};

use super::definition::PackageDefinition;
use super::Package;
use crate::error::*;

impl Package {
    async fn build_package(&self) {
        let build_script = fs::canonicalize(Path::new(&self.info.build.script)).unwrap();

        // TODO: add support for multiple cpu architectures
        process::Command::new("fakeroot")
            .current_dir("build/source/")
            .env(
                "DESTDIR",
                fs::canonicalize("build/dest/").unwrap().to_str().unwrap(),
            )
            .env("SOURCE", "build/source/")
            .env("VERSION", &self.info.package.version)
            .env("TARGET_ARCH", "x86_64")
            .arg("/bin/sh")
            .arg("-c")
            .arg(build_script.to_str().unwrap())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    async fn generate_pkgdef(&self) {
        let mut def = PackageDefinition::new(&self.info);

        // Read all files from dest directory and add them to the definition
        for entry in glob::glob("build/dest/**/*").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let dst = path.strip_prefix("build/dest/").unwrap();

                    // Check if file is a symlink
                    let mode = if path.symlink_metadata().unwrap().file_type().is_symlink() {
                        let symlink = path.symlink_metadata().unwrap();

                        symlink.permissions().mode()
                    } else {
                        0o644
                    };

                    def.add_file(format!("/{}", dst.display()), mode);
                }
                Err(e) => println!("{:?}", e),
            }
        }

        let mut def_file = fs::File::create("build/dest/PKGDEF").unwrap();
        def_file
            .write_all(&bincode::serialize(&def).unwrap())
            .unwrap();
    }

    async fn build_pkg(&self) {
        // Lets build the archive
        let archive_file = fs::File::create(format!(
            "build/{}-{}.gem",
            self.info.package.name, self.info.package.version
        ))
        .unwrap();

        let encoder = zstd::Encoder::new(archive_file, zstd::zstd_safe::max_c_level())
            .unwrap()
            .auto_finish();

        let mut archive = tar::Builder::new(encoder);

        archive.follow_symlinks(false);
        archive.append_dir_all("", "build/dest/").unwrap();
        archive.finish().unwrap();
    }

    pub async fn build(&self) -> Result<PathBuf> {
        // Create the build directory
        fs::create_dir_all("build/source/").unwrap();
        fs::create_dir_all("build/dest/").unwrap();

        self.generate_pkgdef().await;
        self.build_package().await;
        self.build_pkg().await;

        Ok(PathBuf::from(format!(
            "build/{}-{}.gem",
            self.info.package.name, self.info.package.version
        )))
    }
}
