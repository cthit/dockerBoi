pub mod repo;
pub mod repo_list;

use crate::registry::RepoName;
use seed::{log, Url};

pub enum Page {
    RepoList(repo_list::Model),
    Repo(repo::Model),
    NotFound,
}

#[derive(Debug)]
pub enum Route {
    RepoList,
    Repo(RepoName),
    NotFound,
}

pub fn route(mut url: Url) -> Route {
    log(&url);
    match url.remaining_path_parts().as_slice() {
        [] | [""] => Route::RepoList,
        ["repo", "_", name] => Route::Repo(RepoName {
            namespace: None,
            name: name.to_string(),
        }),
        ["repo", namespace, name] => Route::Repo(RepoName {
            namespace: Some(namespace.to_string()),
            name: name.to_string(),
        }),
        _ => Route::NotFound,
    }
}
