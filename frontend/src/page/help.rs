use seed::prelude::*;
use seed::{raw, C, div};

pub fn view<Msg>() -> Node<Msg>{
    div![C!["help"], raw![&markdown::to_html(include_str!("help.md"))]]
}