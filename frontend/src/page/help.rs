use seed::prelude::*;
use seed::{div, raw, C};

pub fn view<Msg>() -> Node<Msg> {
    div![
        C!["help", "list"],
        raw![&markdown::to_html(include_str!("help.md"))]
    ]
}
