use yew::{function_component, html, AttrValue, Callback, Html};
use yewdux::prelude::use_store;

use crate::{
    components::{
        one_post::BBPost,
        title::{BBTitle, BBTitleLevel},
    },
    store::AppState,
};

#[function_component]
pub fn AllPosts() -> Html {
    let (store, _dispatch) = use_store::<AppState>();

    let on_post_edit = {
        Callback::from(|event: AttrValue| {
            gloo::console::log!(event.to_string());
        })
    };

    html! {
        <main>
            <BBTitle level={BBTitleLevel::Two}>{"Posts"}</BBTitle>
            {
                store.posts.iter().map(|post| html!{
                    <BBPost post={post.clone()} onedit={on_post_edit.clone()} />
                }).collect::<Html>()
            }
        </main>
    }
}
