use crate::request::Repo;
use crate::Requestable;

const PROXY_HOSTNAME: &str = "octolotl.axodotdev.host";

#[derive(Debug)]
pub struct FundingFile {
    repo: Repo,
}

impl FundingFile {
    pub fn new(owner: &str, name: &str) -> Self {
        Self {
            repo: Repo::new(owner, name),
        }
    }
}

impl Requestable for FundingFile {
    fn github_url(&self) -> String {
       format!("https://api.github.com/repos/{}/{}/contents/.github/FUNDING.yml", self.repo.owner, self.repo.name)
    }
    fn proxy_url(&self) -> String {
       format!("https://{}/funding/{}/{}", &PROXY_HOSTNAME, self.repo.owner, self.repo.name)
    }
}