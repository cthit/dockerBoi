//! The page for viewing a single repository and its tags

use crate::registry::{self, RepoName};
use seed::browser::fetch;
use seed::prelude::*;
use seed::{div, error, h2, a, C};

pub struct Model {
    repo: RepoName,
    tags: Vec<String>,
}

#[derive(Debug)]
pub enum Msg {
    FetchedTags(fetch::Result<Vec<String>>),
    CopyLink(String),
}


fn copy_to_clipboard(text: &str) {
    seed::window().navigator().clipboard().write_text(text);
}

pub fn init(repo: RepoName, orders: &mut impl Orders<Msg>) -> Model {
    {
        let repo = repo.clone();
        orders.perform_cmd(async move { Msg::FetchedTags(registry::get_image_tags(repo).await) });
    }
    Model { repo, tags: vec![] }
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTags(result) => match result {
            Ok(tags) => model.tags = tags,
            Err(e) => {
                error!(e);
            }
        },
        Msg::CopyLink(text) => copy_to_clipboard(&text),
    }
}

pub fn view(model: &Model) -> Node<Msg> {

    let view_card = |tag: &String| {
        let link: String = format!("localhost:8080/{}:{}", model.repo,tag);
        div![
            C!["repo_card"],
            a![C!["repo_card_header"], tag],
            a![
                C!["repo_link"],
                link.clone(),
                ev(Ev::Click, |_| Msg::CopyLink(link)),
            ]
        ]
    };
    div![
        h2![C!["repo-name"], &model.repo.to_string()],
        model.tags.iter().map(view_card)
    ]
}
