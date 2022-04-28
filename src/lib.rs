use std::{error::Error, fmt::Display};

use ironcurtain_model::courses::{Course, Page};
use parse_link_header::parse_with_rel;
use reqwest::header::ToStrError;

pub mod models;

pub struct Client {
    full_url: String,
    inner_client: reqwest::Client,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    async fn get(&self, path: &str) -> reqwest::Result<reqwest::Response> {
        self.inner_client
            .get(format!("{}{}", self.full_url, path))
            .send()
            .await
    }

    async fn post(&self, path: &str, body: String) -> reqwest::Result<reqwest::Response> {
        self.inner_client
            .post(format!("{}{}", self.full_url, path))
            .body(body)
            .send()
            .await
    }

    pub async fn get_courses(&self) -> Result<Page<Course>, CanvasError> {
        let response = self.get("courses").await?;
        let headers = response.headers();
        let courses = self.get("courses").await?.json::<Vec<Course>>().await?;
        header_to_page(&response, headers.get("link"), courses)
    }
}

fn header_to_page<T>(
    response: &reqwest::Response,
    header: Option<&reqwest::header::HeaderValue>,
    items: Vec<T>,
) -> Result<Page<T>, CanvasError> {
    let header = header.map_or("", |x| x.to_str().map_or("", |x| x));
    let links = parse_with_rel(header)?;
    Ok(Page {
        items,
        current: links
            .get("current")
            .map_or_else(|| response.url().clone(), |x| x.uri.clone()),
        next: links.get("next").map(|x| x.uri.clone()),
        prev: links.get("prev").map(|x| x.uri.clone()),
        first: links
            .get("first")
            .map_or_else(|| response.url().clone(), |x| x.uri.clone()),
        last: links.get("last").map(|x| x.uri.clone()),
    })
}

#[derive(Debug)]
pub struct CanvasError {
    kind: CanvasErrorKind,
    message: String,
}

#[derive(Debug)]
enum CanvasErrorKind {
    Request,
    HeaderParse,
    LinkParse,
}

impl Display for CanvasErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<reqwest::Error> for CanvasError {
    fn from(err: reqwest::Error) -> Self {
        Self {
            kind: CanvasErrorKind::Request,
            message: err.to_string(),
        }
    }
}

impl From<ToStrError> for CanvasError {
    fn from(err: ToStrError) -> Self {
        Self {
            kind: CanvasErrorKind::HeaderParse,
            message: err.to_string(),
        }
    }
}

impl From<parse_link_header::Error> for CanvasError {
    fn from(err: parse_link_header::Error) -> Self {
        Self {
            kind: CanvasErrorKind::LinkParse,
            message: err.to_string(),
        }
    }
}

impl Display for CanvasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CanvasError [kind=\'{}\', message=\'{}\']",
            self.kind, self.message
        )
    }
}

impl Error for CanvasError {}

#[derive(Default)]
pub struct ClientBuilder {
    base_url: String,
    auth_token: String,
}

impl ClientBuilder {
    pub fn build(self) -> Client {
        let auth_header = format!("Bearer {}", self.auth_token.trim());
        let mut headers = reqwest::header::HeaderMap::new();

        headers.append(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(auth_header.as_str())
                .expect("Could not convert authorization header to string"),
        );

        Client {
            full_url: format!("https://{}/api/v1/", self.base_url.trim()),
            inner_client: reqwest::Client::builder()
                .cookie_store(true)
                .default_headers(headers)
                .build()
                .expect("Internal reqwest client failed to build"),
        }
    }

    pub fn set_url(&mut self, base_url: String) -> &mut Self {
        self.base_url = base_url;
        self
    }

    pub fn set_token(&mut self, token: String) -> &mut Self {
        self.auth_token = token;
        self
    }
}
