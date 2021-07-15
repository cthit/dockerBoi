use crate::page::{self, route, Page, Route};
use seed::prelude::*;
use seed::{log, C, a, div, attrs};

pub struct Model {
    route: Route,
    page: Page,
}

#[derive(Debug)]
pub enum Msg {
    NavigateTo(Route),

    RepoListMsg(page::repo_list::Msg),
    RepoMsg(page::repo::Msg),
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // set up routing
    orders
        .subscribe(|subs::UrlChanged(url)| Msg::NavigateTo(route(url)))
        .notify(subs::UrlChanged(url));

    Model {
        page: Page::NotFound,
        route: Route::NotFound,
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    log!(&msg);
    match msg {
        Msg::NavigateTo(route) => {
            model.route = route;
            model.page = match &model.route {
                Route::NotFound => Page::NotFound,
                Route::Help => Page::Help,
                Route::RepoList => {
                    Page::RepoList(page::repo_list::init(&mut orders.proxy(Msg::RepoListMsg)))
                }
                Route::Repo(name) => Page::Repo(page::repo::init(
                    name.clone(),
                    &mut orders.proxy(Msg::RepoMsg),
                )),
            };
        }

        Msg::RepoListMsg(msg) => {
            if let Page::RepoList(model) = &mut model.page {
                page::repo_list::update(msg, model, &mut orders.proxy(Msg::RepoListMsg));
            }
        }

        Msg::RepoMsg(msg) => {
            if let Page::Repo(model) = &mut model.page {
                page::repo::update(msg, model, &mut orders.proxy(Msg::RepoMsg));
            }
        }
    }
}

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        seed::div![
            C!["header"],
            a![
                C!["header_text"],
                "dockerBoi", 
                attrs! {
                    At::Href => "/"
                },
            ]
        ],
        seed::a![
            C!["help_button"],
            attrs! {
                At::Href => "/help",
            },
            "?",
        ],
        match &model.page {
            Page::Repo(model) => page::repo::view(model).map_msg(Msg::RepoMsg),
            Page::RepoList(model) => page::repo_list::view(model).map_msg(Msg::RepoListMsg),
            Page::Help => page::help::view(),
            Page::NotFound => seed::h1!["Not Found"],
        },
    ]
}
