use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Request error")]
    RequestFailure {
        #[from]
        source: reqwest::Error,
    },
    #[error("Failed to parse header")]
    HeaderParse {
        #[from]
        source: reqwest::header::ToStrError,
    },
    #[error("Failed to parse pagination links")]
    LinkParse {
        #[from]
        source: parse_link_header::Error,
    },
    #[error("URL parsing error")]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
}
