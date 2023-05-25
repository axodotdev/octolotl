use crate::request::Repo;
use crate::Requestable;

const PROXY_HOSTNAME: &str = "octolotl.axodotdev.host";

#[derive(Debug)]
pub struct Releases {
    repo: Repo,
}

impl Releases {
    pub fn new(owner: &str, name: &str) -> Self {
        Self {
            repo: Repo::new(owner, name),
        }
    }
}

impl Requestable for Releases {
    fn github_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/releases",
            &self.repo.owner, &self.repo.name
        )
    }

    fn proxy_url(&self) -> String {
        format!(
            "https://{}/releases/{}/{}",
            PROXY_HOSTNAME, &self.repo.owner, &self.repo.name
        )
    }
}
