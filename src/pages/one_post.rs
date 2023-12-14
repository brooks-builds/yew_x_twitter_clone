use yew::{function_component, html, use_effect, Html, Properties};
use yewdux::prelude::use_store;

use crate::{
    components::{
        one_post::BBPost,
        title::{BBTitle, BBTitleLevel},
    },
    store::AppState,
};

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub id: i32,
}

#[function_component]
pub fn OnePost(props: &Props) -> Html {
    let (store, _dispatch) = use_store::<AppState>();

    let Some(post) = store.get_post_by_id(props.id) else {
        return html! {
            <></>
        };
    };

    html! {
        <main>
            <BBPost post={post.clone()}/>
        </main>
    }
}
