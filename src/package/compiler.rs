use std::path::Path;
use std::{fs, io::Write, path::PathBuf};

use git2::Repository;
use hyper::body::HttpBody;
use hyper::Client;
use hyper::Uri;

use super::Package;
use crate::error::*;

impl Package {
    async fn pull_sources(&self) -> Result<()> {
        let client = Client::new();

        for (name, source) in &self.info.sources {
            let is_git = source.branch.is_some();

            let source_path = format!("./build/source/{}", name);
            if is_git {
                let repo = if Path::new(&source_path).exists() {
                    match Repository::open(source_path) {
                        Ok(repo) => repo,
                        Err(e) => Err(ErrorKind::GitError(e))?,
                    }
                } else {
                    match Repository::clone(&source.url, source_path) {
                        Ok(repo) => repo,
                        Err(e) => Err(ErrorKind::GitError(e))?,
                    }
                };

                let branch = source.branch.as_ref().unwrap();
                match repo.set_head(&format!("refs/tags/{}", branch)) {
                    Ok(it) => it,
                    Err(err) => Err(ErrorKind::GitError(err))?,
                };

                match repo.checkout_head(None) {
                    Ok(it) => it,
                    Err(err) => Err(ErrorKind::GitError(err))?,
                };

                let obj = repo
                    .find_object(repo.head().unwrap().target().unwrap(), None)
                    .unwrap();
                match repo.reset(&obj, git2::ResetType::Hard, None) {
                    Ok(it) => it,
                    Err(err) => Err(ErrorKind::GitError(err))?,
                };
            } else {
                // Download the source
                let url: Uri = match source.url.parse() {
                    Ok(it) => it,
                    Err(err) => Err(ErrorKind::InvalidUri(err))?,
                };

                let mut resp = match client.get(url).await {
                    Ok(it) => it,
                    Err(err) => Err(ErrorKind::HyperError(err))?,
                };

                let mut file = match fs::File::create(source_path) {
                    Ok(it) => it,
                    Err(err) => Err(ErrorKind::IoError(err))?,
                };

                // Download the file and write it to the build directory
                while let Some(chunk) = resp.body_mut().data().await {
                    match chunk {
                        Ok(chunk) => match file.write_all(&chunk) {
                            Err(err) => {
                                Err(ErrorKind::IoError(err))?;
                            }
                            _ => (),
                        },
                        Err(err) => Err(ErrorKind::HyperError(err))?,
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn build(&self) -> Result<PathBuf> {
        // Create the build directory
        fs::create_dir_all("build/source/").unwrap();
        fs::create_dir_all("build/dest/").unwrap();

        // Pull the sources
        self.pull_sources().await?;

        Ok(PathBuf::new())
    }
}
