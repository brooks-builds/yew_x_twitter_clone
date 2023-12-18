use super::textarea::BBTextArea;
use std::ops::Deref;
use stylist::{style, yew::styled_component};
use web_sys::SubmitEvent;
use yew::{html, use_state, AttrValue, Callback, Html, Properties};

use crate::components::title::{BBTitle, BBTitleLevel};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub oncreatepost: Callback<AttrValue>,
    #[prop_or_else(|| "Create Post".into())]
    pub title: AttrValue,
}

#[styled_component]
pub fn CreatePost(props: &Props) -> Html {
    let form = style! {
        display: flex;
        flex-direction: column;

        button {
            margin-top: 1rem;
            width: 6rem;
        }
    }
    .unwrap();

    let new_post_text = use_state(|| AttrValue::default());

    let onsubmit = {
        let new_post_text = new_post_text.clone();
        let props_oncreatepost = props.oncreatepost.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let post_text = new_post_text.deref().clone();

            props_oncreatepost.emit(post_text);
            new_post_text.set(AttrValue::default());
        })
    };

    let bb_textarea_onchange = {
        let new_post_text = new_post_text.clone();

        Callback::from(move |post_text: AttrValue| {
            new_post_text.set(post_text);
        })
    };

    html! {
            <form class={form} {onsubmit}>
                <BBTitle level={BBTitleLevel::Two}>{props.title.clone()}</BBTitle>
                <BBTextArea onchange={bb_textarea_onchange} value={new_post_text.deref().clone()} />
                <button type={"submit"}>{"Submit post"}</button>
            </form>
    }
}
