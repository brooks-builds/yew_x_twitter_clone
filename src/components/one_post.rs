use crate::router::Route;
use stylist::{style, yew::styled_component};
use yew::{html, Html, Properties};
use yew_router::prelude::Link;

use crate::store::Post;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub post: Post,
}

#[styled_component]
pub fn BBPost(props: &Props) -> Html {
    let style = style!(
        r#"
            border: 3px solid skyblue;
            padding: 2rem;

            :hover {
                background-color: gray;
            }
        "#
    )
    .unwrap();

    let link_style = style!(
        r#"
            text-decoration: none !important;
            color: black;
        "#
    )
    .unwrap();

    html! {
        <Link<Route> to={Route::OnePost { id: props.post.id }} classes={link_style}>
            <div class={style}>
                <p>{props.post.text.clone()}</p>
                <p>{"likes:"} {props.post.likes}</p>
                <p>{"Responses:"} {props.post.response_count}</p>
            </div>
        </Link<Route>>
    }
}
