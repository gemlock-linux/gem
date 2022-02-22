#[derive(Debug)]
pub enum ErrorKind {
    TomlError(toml::de::Error),
    VersionError(String),
    IoError(std::io::Error),
}

pub type Result<T> = std::result::Result<T, ErrorKind>;
