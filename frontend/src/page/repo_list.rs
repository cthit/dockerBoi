//! The page for viewing the list of all repositories

use crate::registry::{self, RepoName};
use seed::browser::fetch;
use seed::error;
use seed::prelude::*;
use seed::{a, attrs, div, C};

pub struct Model {
    repositories: Vec<RepoName>,
}

#[derive(Debug)]
pub enum Msg {
    FetchedRepos(fetch::Result<Vec<RepoName>>),
}

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedRepos(registry::get_repositories().await) });

    Model {
        repositories: vec![],
    }
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedRepos(result) => match result {
            Ok(repos) => model.repositories = repos,
            Err(e) => {
                error!(e);
            }
        },
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let view_card = |name: &RepoName| {
        a![
            C!["repo_card"],
            attrs! {
                At::Href => match name.namespace {
                    Some(_) => format!("/repo/{}", name),
                    None => format!("/repo/_/{}", name),
                }
            },
            div![
                C!["repo_card_header"],
                
                format!("{}", name),
            ]
        ]
    };
    div![model.repositories.iter().map(view_card)]
}
