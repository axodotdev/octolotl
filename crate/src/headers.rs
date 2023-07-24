use reqwest::header::{HeaderMap, ACCEPT, USER_AGENT};

#[derive(Clone, Debug)]
pub struct Headers {
    pub inner: HeaderMap,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            inner: HeaderMap::new(),
        }
    }

    pub fn proxy(&mut self) -> Self {
        self.add_user_agent().clone()
    }

    pub fn github(&mut self) -> Self {
        self.add_user_agent().add_github_api().clone()
    }

    fn add_github_api(&mut self) -> &mut Self {
        self.inner.insert(
            ACCEPT,
            "application/vnd.github+json"
                .parse()
                .expect("accept header was invalid"),
        );
        self.inner.insert(
            "X-GitHub-Api-Version",
            "2022-11-28"
                .parse()
                .expect("api version header was invalid"),
        );
        self
    }

    fn add_user_agent(&mut self) -> &mut Self {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        const NAME: &str = env!("CARGO_PKG_NAME");
        let value = format!("{}-{}", NAME, VERSION);
        self.inner
            .insert(USER_AGENT, value.parse().expect("user agent was invalid"));
        self
    }
}
