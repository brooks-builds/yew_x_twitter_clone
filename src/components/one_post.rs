use crate::{components::textarea::BBTextArea, router::Route};
use stylist::{style, yew::styled_component};
use yew::{classes, html, use_state, Callback, Html, Properties};
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
    let editing_state = use_state(|| false);

    let style = style!(
        r#"
            border: 3px solid skyblue;
            padding: 2rem;
            display: flex;
            flex-direction: column;
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

    let edit_button_style = style!(
        r#"
            margin-left: auto;
        "#
    )
    .unwrap();

    let edit_button_onclick = {
        let editing_state = editing_state.clone();

        Callback::from(move |_event| {
            editing_state.set(!*editing_state);
        })
    };

    let post_html = html! {
            <div class={style.clone()}>
                {
                    if *editing_state {
                        html! {
                            <BBTextArea value={props.post.text.clone()}/>
                        }
                    } else {
                        html! {
                            <p>{props.post.text.clone()}</p>
                        }
                    }
                }
                <p>{"likes:"} {props.post.likes}</p>
                <p>{"Responses:"} {props.post.response_count}</p>
                <button class={edit_button_style} onclick={edit_button_onclick}>{if *editing_state {"save"} else {"edit"}}</button>
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
