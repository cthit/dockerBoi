use seed::browser::fetch::{self, fetch, header::Header, Request, Response};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RepoName {
    pub namespace: Option<String>,
    pub name: String,
}

pub async fn get_repositories() -> fetch::Result<Vec<RepoName>> {
    #[derive(Deserialize)]
    struct RegistryResponse {
        repositories: Vec<String>,
    }

    let response: RegistryResponse = get_with_token("/v2/_catalog")
        .await?
        .check_status()?
        .json()
        .await?;

    Ok(response
        .repositories
        .into_iter()
        .map(|name| RepoName::from_str(&name))
        .collect::<Result<Vec<_>, _>>()
        .expect("todo"))
}

pub async fn get_with_token(url: &str) -> fetch::Result<Response> {
    #[derive(Deserialize)]
    struct TokenResponse {
        token: String,
    }

    let maybe_response = fetch(url).await?;
    let status = maybe_response.status();
    if status.code == 401 {
        let www_auth = maybe_response
            .raw_response()
            .headers()
            .get("www-authenticate");
        let www_auth = match www_auth {
            Ok(Some(www_auth)) => www_auth,
            _ => todo!("handle error"),
        };

        let www_auth = www_auth.strip_prefix("Bearer ").unwrap();

        let params: HashMap<_, _> = www_auth
            .split(",")
            .flat_map(|kv| kv.split_once("="))
            .map(|(k, v)| (k, v.trim_matches('"')))
            .collect();

        let realm = *params.get("realm").unwrap();

        let token: TokenResponse = fetch(format!(
            "{}?service={}&scope={}",
            realm,
            params.get("service").unwrap(),
            params.get("scope").unwrap()
        ))
        .await?
        .check_status()?
        .json()
        .await?;

        let request = Request::new(url).header(Header::bearer(token.token));
        fetch(request).await
    } else {
        Ok(maybe_response)
    }
}

pub async fn get_image_tags(name: RepoName) -> fetch::Result<Vec<String>> {
    #[derive(Deserialize)]
    struct RegistryResponse {
        #[allow(dead_code)]
        name: String,
        tags: Vec<String>,
    }

    async {
        let response: RegistryResponse = get_with_token(&format!("/v2/{}/tags/list", name))
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
