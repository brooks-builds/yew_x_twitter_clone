use stylist::yew::styled_component;
use yew::{html, AttrValue, Callback, Html};
use yewdux::prelude::use_store;

use crate::{
    api,
    components::{
        all_posts::AllPosts,
        create_post::CreatePost,
        title::{BBTitle, BBTitleLevel},
    },
    store::{AppState, Post},
};

#[styled_component]
pub fn Home() -> Html {
    let (_store, dispatch) = use_store::<AppState>();

    let oncreatepost = Callback::from(move |post: AttrValue| {
        let dispatch = dispatch.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let created_post = match api::create_post(post).await {
                Ok(post) => post,
                Err(error) => {
                    gloo::console::error!("Error creating post", error.to_string());
                    return;
                }
            };

            let post = Post::new(created_post.id, created_post.text);
            dispatch.reduce_mut(move |state| {
                state.posts.insert(0, post);
            });
        })
    });

    html! {
        <>
            <BBTitle center={true} level={BBTitleLevel::One}>{"X/Twitter Clone"}</BBTitle>
            <CreatePost {oncreatepost}/>
            <AllPosts />
        </>
    }
}
