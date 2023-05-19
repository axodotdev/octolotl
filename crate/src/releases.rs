use crate::Requestable;

pub struct Releases {
    repo_owner: String,
    repo_name: String,
}

impl Requestable for Releases {
    fn github_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/releases",
            &self.repo_owner, &self.repo_name
        )
    }

    fn proxy_url(&self) -> String {
        format!(
            "https://octolotl.axodotdev.host/releases/{}/{}",
            &self.repo_owner, &self.repo_name
        )
    }
}
