use yew::{
    function_component, html, use_effect_with, use_state, AttrValue, Callback, Html, Properties,
};
use yewdux::prelude::use_store;

use crate::{
    api,
    components::{
        create_post::CreatePost,
        one_post::BBPost,
        title::{BBTitle, BBTitleLevel},
    },
    store::AppState,
};

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub id: u32,
}

#[function_component]
pub fn OnePost(props: &Props) -> Html {
    let (store, dispatch) = use_store::<AppState>();
    let loaded_state = use_state(|| false);

    {
        let id = props.id;
        let dispatch = dispatch.clone();

        use_effect_with(*loaded_state, move |_loaded| {
            let dispatch = dispatch.clone();
            let loaded_state = loaded_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match api::get_one_post(id).await {
                    Ok(Some(post)) => {
                        dispatch.reduce_mut(move |state| state.add_post(post.into()));
                        loaded_state.set(true);
                    }
                    Err(error) => {
                        gloo::console::error!("Error getting one post", error.to_string());
                    }
                    Ok(None) => unreachable!(),
                }
            });

            || {}
        })
    }

    let oncreatereply = {
        let dispatch = dispatch.clone();
        let parent_id = props.id;

        dispatch.reduce_mut_future_callback_with(move |state, reply: AttrValue| {
            Box::pin(async move {
                match api::create_post(reply.clone(), Some(parent_id)).await {
                    Ok(reply_post) => {
                        state.add_reply(reply_post.into(), parent_id);
                    }
                    Err(error) => {
                        gloo::console::error!("Error replying to post", error.to_string());
                    }
                }
            })
        })
    };

    let Some(post) = store.get_post_by_id(props.id) else {
        return html! {
            <></>
        };
    };

    let onedit = {
        Callback::from(|edited_text: AttrValue| {
            gloo::console::log!(edited_text.to_string());
        })
    };

    html! {
        <main>
            <BBPost post={post.clone()} disable_navigation={true} canedit={true} {onedit} />
            <CreatePost title="Reply" oncreatepost={oncreatereply} />
            <BBTitle level={BBTitleLevel::Two} center={true}>{"Comments"}</BBTitle>
            {
                post.responses.iter().map(|response| html! { <BBPost post={response.clone()} disable_navigation={true} / >}).collect::<Vec<Html>>()
            }
        </main>
    }
}
