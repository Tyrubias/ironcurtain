use error::Result;
use reqwest::{ClientBuilder, Url};

pub mod error;
#[derive(Debug, Clone)]
pub struct Canvas {
    api_url: Url,
    client: reqwest::Client,
}

impl Canvas {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct CanvasBuilder<'a> {
    base_url: &'a str,
    auth_token: &'a str,
}

impl<'a> CanvasBuilder<'a> {
    pub fn build(self) -> Result<Canvas> {
        use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

        let auth_header = format!("Bearer {}", self.auth_token.trim());
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(auth_header.as_str())
                .expect("Could not convert authorization header to string"),
        );

        Ok(Canvas {
            api_url: Url::parse(self.base_url).and_then(|url| url.join("/api/v1/"))?,
            client: reqwest::Client::builder()
                .cookie_store(true)
                .default_headers(headers)
                .build()?,
        })
    }

    pub fn set_url(mut self, base_url: &'a str) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn set_token(mut self, token: &'a str) -> Self {
        self.auth_token = token;
        self
    }
}
