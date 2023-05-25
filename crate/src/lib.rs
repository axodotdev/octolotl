use reqwest::header::{HeaderMap, USER_AGENT};
use reqwest::Response;

use errors::*;

mod errors;
pub use errors::OctolotlError;
pub mod request;

pub trait Requestable {
    fn github_url(&self) -> String;
    fn proxy_url(&self) -> String;
}

pub struct Request<'a> {
    client: reqwest::Client,
    item: &'a dyn Requestable,
}

impl<'a> Request<'a> {
    fn new(item: &'a impl Requestable) -> Request<'a> {
        Self {
            client: reqwest::Client::new(),
            item,
        }
    }

    pub async fn send(item: &'a impl Requestable, proxy: bool) -> Result<Response> {
        let request = Self::new(item);
        if proxy {
            match request.via_proxy().await {
                Ok(res) => Ok(res),
                Err(e) => {
                    warn_with_proxy_error(e);
                    request.via_github().await
                }
            }
        } else {
            request.via_github().await
        }
    }

    async fn via_proxy(&self) -> Result<Response> {
        Ok(request(&self.item.proxy_url(), &self.client)
            .await?
            .error_for_status()?)
    }

    async fn via_github(&self) -> Result<Response> {
        Ok(request(&self.item.github_url(), &self.client)
            .await?
            .error_for_status()?)
    }
}

async fn request(url: &str, client: &reqwest::Client) -> Result<Response> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        user_agent().parse().expect("user agent was invalid"),
    );
    Ok(client.get(url).headers(headers).send().await?)
}

fn user_agent() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");
    format!("{}-{}", NAME, VERSION)
}

fn warn_with_proxy_error(e: OctolotlError) {
    println!(
        "Fetching from proxy encountered an error:\n {e}.\n Falling back to GitHub directly..."
    )
}
