use stylist::yew::styled_component;
use yew::{html, AttrValue, Callback, Html};

use crate::components::{
    create_post::CreatePost,
    title::{BBTitle, BBTitleLevel},
};

#[styled_component]
pub fn Home() -> Html {
    let oncreatepost = Callback::from(|post: AttrValue| {
        gloo::console::log!(post.to_string());
    });

    html! {
        <>
            <BBTitle center={true} level={BBTitleLevel::One}>{"X/Twitter Clone"}</BBTitle>
            <CreatePost {oncreatepost}/>
        </>
    }
}
