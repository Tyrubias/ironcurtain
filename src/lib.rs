use ironcurtain_model::courses::{Course, Page};
use reqwest::header::HeaderValue;

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

    pub async fn get_courses(&self) -> Result<Page<Course>, reqwest::Error> {
        let response = self.get("courses").await?;
        let headers = response.headers();
        let courses = self.get("courses").await?.json::<Vec<Course>>().await?;

        match headers.get("link") {
            Some(_value) => Ok(Page {
                items: courses,
                current: response.url().to_string(),
                next: None,
                prev: None,
                first: response.url().to_string(),
                last: None,
            }),
            None => Ok(Page {
                items: courses,
                current: response.url().to_string(),
                next: None,
                prev: None,
                first: response.url().to_string(),
                last: None,
            }),
        }
    }
}

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

fn parse_link(link_header: &HeaderValue) {
    let full_link = link_header.to_str().unwrap().to_owned();
    let link_segs: Vec<&str> = full_link.split(',').collect();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
