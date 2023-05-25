use miette::Diagnostic;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, OctolotlError>;

#[derive(Debug, Diagnostic, Error)]
pub enum OctolotlError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Failed to parse repo: {repo}")]
    #[diagnostic(help("Please make sure this is correct."))]
    RepoParseError {
        repo: String,
        #[diagnostic_source]
        details: miette::Report,
    },

    #[error("Failed fetching releases from Github.")]
    GithubReleasesFetchError {
        #[source]
        details: reqwest::Error,
    },

    #[error("Failed parsing response when fetching releases from Github.")]
    GithubReleaseParseError {
        #[source]
        details: reqwest::Error,
    },
}
