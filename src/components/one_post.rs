use stylist::{style, yew::styled_component};
use yew::{html, Html, Properties};

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
        "#
    )
    .unwrap();

    html! {
        <div class={style}>
            <p>{props.post.text.clone()}</p>
            <p>{"likes:"} {props.post.likes}</p>
            <p>{"Responses:"} {props.post.response_count}</p>
        </div>
    }
}
