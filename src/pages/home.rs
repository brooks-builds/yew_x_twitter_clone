use stylist::yew::styled_component;
use yew::{html, AttrValue, Callback, Html};

use crate::{
    api,
    components::{
        create_post::CreatePost,
        title::{BBTitle, BBTitleLevel},
    },
};

#[styled_component]
pub fn Home() -> Html {
    let oncreatepost = Callback::from(|post: AttrValue| {
        wasm_bindgen_futures::spawn_local(async move {
            let created_post = match api::create_post(post).await {
                Ok(post) => post,
                Err(error) => {
                    gloo::console::error!("Error creating post", error.to_string());
                    return;
                }
            };

            gloo::console::log!(
                "created post with id, and text",
                created_post.id,
                created_post.text.to_string()
            );
        })
    });

    html! {
        <>
            <BBTitle center={true} level={BBTitleLevel::One}>{"X/Twitter Clone"}</BBTitle>
            <CreatePost {oncreatepost}/>
        </>
    }
}
