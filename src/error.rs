#[derive(Debug)]
pub enum ErrorKind {
    TomlError(toml::de::Error),
    VersionError(String),
    IoError(std::io::Error),
    HyperError(hyper::Error),
    InvalidUri(hyper::http::uri::InvalidUri),
}

pub type Result<T> = std::result::Result<T, ErrorKind>;
