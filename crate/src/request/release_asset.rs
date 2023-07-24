use crate::request::Repo;
use crate::Requestable;
use crate::PROXY_HOSTNAME;

#[derive(Debug)]
pub struct ReleaseAsset {
    repo: Repo,
    file: String,
    tag: String,
}

impl ReleaseAsset {
    pub fn new(owner: &str, name: &str, tag: &str, file: &str) -> Self {
        Self {
            repo: Repo::new(owner, name),
            file: file.to_string(),
            tag: tag.to_string(),
        }
    }
}

impl Requestable for ReleaseAsset {
    fn github_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/releases/download/{}/{}",
            &self.repo.owner, &self.repo.name, &self.tag, &self.file
        )
    }

    fn proxy_url(&self) -> String {
        format!(
            "https://{}/downloads/{}/{}/{}/{}",
            &PROXY_HOSTNAME, self.repo.owner, &self.repo.name, &self.tag, &self.file
        )
    }
}
