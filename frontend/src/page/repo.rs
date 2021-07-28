//! The page for viewing a single repository and its tags

use crate::registry::{self, RepoName};
use seed::browser::fetch;
use seed::prelude::*;
use seed::{a, attrs, button, div, error, h2, img, input, span, C};
use semver::Version;
use std::cmp::Ordering;

pub struct Model {
    repo: RepoName,
    tags: Vec<Tag>,
}

struct Tag {
    name: String,
    copied: bool,
}

#[derive(Debug)]
pub enum Msg {
    FetchedTags(fetch::Result<Vec<String>>),
    CopyLink(usize, String),
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

fn is_latest(a: &str, b: &str) -> Ordering {
    match (a, b) {
        ("latest", _) => Ordering::Less,
        (_, "latest") => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

fn is_version(a: &str, b: &str) -> Ordering {
    match (Version::parse(a).is_ok(), Version::parse(b).is_ok()) {
        (true, true) => b.cmp(a),
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTags(result) => match result {
            Ok(mut tags) => {
                tags.sort_by(|a, b| is_latest(a, b).then(is_version(a, b)).then(a.cmp(b)));
                model.tags = tags
                    .into_iter()
                    .map(|name| Tag {
                        name,
                        copied: false,
                    })
                    .collect()
            }
            Err(e) => {
                error!(e);
            }
        },
        Msg::CopyLink(i, link) => {
            copy_to_clipboard(&link);
            for (j, tag) in model.tags.iter_mut().enumerate() {
                tag.copied = i == j;
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let view_card = |(i, tag): (usize, &Tag)| {
        let link: String = format!("docker.chalmers.it/{}:{}", model.repo, tag.name);
        div![
            C!["repo_card"],
            a![C!["repo_card_header"], &tag.name],
            input![
                C!["repo_link"],
                attrs! {
                    At::Value => link.clone(),
                    At::ReadOnly => true,
                },
            ],
            button![
                C!["copy_button"],
                ev(Ev::Click, move |_| Msg::CopyLink(i, link)),
                if tag.copied {
                    span!["✔️"]
                } else {
                    span![img![attrs! { At::Src => "/images/clipboard.svg" },],]
                },
            ],
        ]
    };
    div![
        div![
            C!["list"],
            h2![C!["repo-name"], &model.repo.to_string()],
            model.tags.iter().enumerate().map(view_card)
        ] // Iter<&Tag> -> Iter<(usize, &Tag)>
    ]
}
