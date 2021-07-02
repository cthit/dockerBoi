use seed::browser::fetch::{self, fetch};
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RepoName {
    pub namespace: Option<String>,
    pub name: String,
}

pub async fn get_repositories() -> fetch::Result<Vec<RepoName>> {
    #[derive(Deserialize)]
    struct RegistryResponse {
        repositories: Vec<String>,
    }

    async {
        let response: RegistryResponse =
            fetch("/v2/_catalog").await?.check_status()?.json().await?;

        Ok(response
            .repositories
            .into_iter()
            .map(|name| RepoName::from_str(&name))
            .collect::<Result<Vec<_>, _>>()
            .expect("todo"))
    }
    .await
}

pub async fn get_image_tags(name: RepoName) -> fetch::Result<Vec<String>> {
    #[derive(Deserialize)]
    struct RegistryResponse {
        #[allow(dead_code)]
        name: String,
        tags: Vec<String>,
    }

    async {
        let response: RegistryResponse = fetch(format!("/v2/{}/tags/list", name))
            .await?
            .check_status()?
            .json()
            .await?;

        Ok(response.tags)
    }
    .await
}

impl Display for RepoName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(namespace) = &self.namespace {
            write!(f, "{}/", namespace)?;
        }
        write!(f, "{}", self.name)
    }
}

impl FromStr for RepoName {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: validate against regex
        Ok(match s.split_once('/') {
            Some((namespace, name)) => RepoName {
                name: name.to_string(),
                namespace: Some(namespace.to_string()),
            },
            None => RepoName {
                namespace: None,
                name: s.to_string(),
            },
        })
    }
}
