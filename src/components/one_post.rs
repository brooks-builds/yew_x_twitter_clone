use crate::router::Route;
use stylist::{style, yew::styled_component};
use yew::{classes, html, Html, Properties};
use yew_router::prelude::Link;

use crate::store::Post;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub post: Post,
    #[prop_or_default]
    pub disable_navigation: bool,
}

#[styled_component]
pub fn BBPost(props: &Props) -> Html {
    let style = style!(
        r#"
            border: 3px solid skyblue;
            padding: 2rem;
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

    let navigation_hover = style!(
        r#"
            div:hover {
                background-color: gray;
            }
        "#
    )
    .unwrap();

    let post_html = html! {
            <div class={style.clone()}>
                <p>{props.post.text.clone()}</p>
                <p>{"likes:"} {props.post.likes}</p>
                <p>{"Responses:"} {props.post.response_count}</p>
            </div>
    };

    if props.disable_navigation {
        post_html
    } else {
        html! {
            <Link<Route> to={Route::OnePost { id: props.post.id }} classes={classes!(link_style, navigation_hover)}>
                {post_html}
            </Link<Route>>
        }
    }
}
