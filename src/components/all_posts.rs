use yew::{function_component, html, AttrValue, Html};
use yewdux::prelude::use_store;

use crate::{
    components::{
        one_post::BBPost,
        title::{BBTitle, BBTitleLevel},
    },
    store::{AppState, Post},
};

#[function_component]
pub fn AllPosts() -> Html {
    let (store, _dispatch) = use_store::<AppState>();

    html! {
        <main>
            <BBTitle level={BBTitleLevel::Two}>{"Posts"}</BBTitle>
            {
                store.posts.iter().map(|post| html!{
                    <BBPost post={post.clone()}/>
                }).collect::<Html>()
            }
        </main>
    }
}
