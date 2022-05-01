use futures::{Future, Stream};
pub use ironcurtain_model as models;
use std::{error::Error, fmt::Display, pin::Pin};

use crate::models::courses::{Course, Page};
use parse_link_header::parse_with_rel;
use reqwest::{header::ToStrError, Url};

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
        let headers = response.headers().clone();
        let url = response.url().clone();
        let courses = response.json::<Vec<Course>>().await?;
        header_to_page(&url, headers.get("link"), courses)
    }
}

fn header_to_page<T>(
    resp_url: &Url,
    header: Option<&reqwest::header::HeaderValue>,
    items: Vec<T>,
) -> Result<Page<T>, CanvasError> {
    let header = header.ok_or_else(CanvasError::default)?.to_str()?;
    let links = parse_with_rel(header)?;
    Ok(Page {
        items,
        current: links
            .get("current")
            .map_or_else(|| resp_url.clone(), |x| x.uri.clone()),
        next: links.get("next").map(|x| x.uri.clone()),
        prev: links.get("prev").map(|x| x.uri.clone()),
        first: links
            .get("first")
            .map_or_else(|| resp_url.clone(), |x| x.uri.clone()),
        last: links.get("last").map(|x| x.uri.clone()),
    })
}

#[derive(Default, Debug)]
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

impl Default for CanvasErrorKind {
    fn default() -> Self {
        CanvasErrorKind::Request
    }
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

pub type Paginator<'a, T> = Pin<Box<dyn Stream<Item = T> + 'a>>;

pub fn paginate<'a, T: 'a, Fut, Request: 'a>(
    req: Request,
    next_url: Url,
) -> Paginator<'a, Result<T, CanvasError>>
where
    T: Unpin,
    Fut: Future<Output = Result<Page<T>, CanvasError>>,
    Request: Fn(Url) -> Fut,
{
    use async_stream::stream;
    Box::pin(stream! {
        let mut page_url = next_url;
        loop {
            let page = req(page_url).await?;
            for item in page.items {
                yield Ok(item);
            }
            if page.next.is_none() {
                break;
            }
            page_url = page.next.unwrap();
        }
    })
}

#[derive(Default)]
pub struct ClientBuilder {
    base_url: String,
    auth_token: String,
}

impl ClientBuilder {
    pub fn build(&self) -> Client {
        let auth_header = format!("Bearer {}", self.auth_token.trim());
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(
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

#[cfg(test)]
mod test {
    #[test]
    fn test_courses() {}
}
