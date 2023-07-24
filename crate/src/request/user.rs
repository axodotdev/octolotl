use std::env;

use reqwest::header::{HeaderMap, AUTHORIZATION};

use crate::headers::Headers;
use crate::Requestable;
use crate::PROXY_HOSTNAME;

#[derive(Debug)]
pub struct User {}

impl User {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Requestable for User {
    fn github_url(&self) -> String {
        "https://api.github.com/user".to_string()
    }

    fn github_headers(&self) -> HeaderMap {
        let mut headers = Headers::new().github().inner;
        headers.append(
            AUTHORIZATION,
            env::var("GITHUB_TOKEN")
                .expect("$GITHUB_TOKEN is not set")
                .parse()
                .expect("authorization header was invalid"),
        );
        headers
    }

    fn proxy_url(&self) -> String {
        format!("https://{}/user", &PROXY_HOSTNAME)
    }
}
