use reqwest::header::HeaderMap;
use reqwest::Response;

use errors::*;

mod errors;
mod headers;
pub mod request;

pub use errors::OctolotlError;
pub use headers::Headers;

const PROXY_HOSTNAME: &str = "octolotl.axodotdev.host";

pub trait Requestable {
    fn github_url(&self) -> String;
    fn proxy_url(&self) -> String;
    fn proxy_headers(&self) -> HeaderMap {
        let mut headers = Headers::new();
        headers.proxy().inner
    }
    fn github_headers(&self) -> HeaderMap {
        let mut headers = Headers::new();
        headers.github().inner
    }
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
        Ok(request(
            &self.item.proxy_url(),
            &self.client,
            self.item.proxy_headers(),
        )
        .await?
        .error_for_status()?)
    }

    async fn via_github(&self) -> Result<Response> {
        Ok(request(
            &self.item.github_url(),
            &self.client,
            self.item.github_headers(),
        )
        .await?
        .error_for_status()?)
    }
}

async fn request(url: &str, client: &reqwest::Client, headers: HeaderMap) -> Result<Response> {
    Ok(client.get(url).headers(headers).send().await?)
}

fn warn_with_proxy_error(e: OctolotlError) {
    println!(
        "Fetching from proxy encountered an error:\n {e}.\n Falling back to GitHub directly..."
    )
}
