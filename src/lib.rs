use api::courses::CourseHandler;
use page::Page;
use parse_link_header::parse_with_rel;
use reqwest::Url;
use serde::Serialize;

use crate::result::Result;

pub mod api;
pub mod error;
pub mod models;
pub mod page;
pub mod result;

#[derive(Debug, Clone)]
pub struct Canvas {
    api_url: Url,
    client: reqwest::Client,
}

impl Canvas {
    pub fn builder<'a>() -> CanvasBuilder<'a> {
        CanvasBuilder::default()
    }

    pub fn courses(&self) -> CourseHandler {
        CourseHandler::new(self)
    }

    pub(crate) async fn get_page<R, A, P>(
        &self,
        route: A,
        parameters: Option<&P>,
    ) -> Result<Page<R>>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: serde::de::DeserializeOwned,
    {
        let mut response = self.client.get(self.absolute_url(route)?);

        if let Some(parameters) = parameters {
            response = response.query(parameters);
        }

        let response = response.send().await?;

        let headers = response.headers().clone();
        let full_url = response.url().clone();
        let items = response.json::<Vec<R>>().await?;

        let link_header = headers
            .get("link")
            .ok_or(error::Error::NoLinkHeader)?
            .to_str()?;
        let links = parse_with_rel(link_header)?;

        Ok(Page {
            items,
            current: links
                .get("current")
                .map_or_else(|| full_url.clone(), |x| x.uri.clone()),
            next: links.get("next").map(|x| x.uri.clone()),
            prev: links.get("prev").map(|x| x.uri.clone()),
            first: links
                .get("first")
                .map_or_else(|| full_url.clone(), |x| x.uri.clone()),
            last: links.get("last").map(|x| x.uri.clone()),
        })
    }

    pub fn absolute_url(&self, url: impl AsRef<str>) -> Result<Url> {
        Ok(self.api_url.join(url.as_ref())?)
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

        let mut auth_header_val = HeaderValue::from_str(auth_header.as_str())
            .expect("Could not convert authorization header to string");

        auth_header_val.set_sensitive(true);

        headers.insert(AUTHORIZATION, auth_header_val);

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
