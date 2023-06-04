mod release_asset;
mod releases;
mod funding;

pub use release_asset::ReleaseAsset;
pub use releases::Releases;

#[derive(Debug)]
struct Repo {
    owner: String,
    name: String,
}

impl Repo {
    fn new(owner: &str, name: &str) -> Self {
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    }
}
