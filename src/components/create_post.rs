use std::ops::Deref;

use stylist::{style, yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlTextAreaElement, SubmitEvent};
use yew::{html, use_state, AttrValue, Callback, Html, Properties};

use crate::components::title::{BBTitle, BBTitleLevel};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub oncreatepost: Callback<AttrValue>,
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
        })
    };

    let textarea_onchange = {
        let new_post_text = new_post_text.clone();
        Callback::from(move |event: Event| {
            let target = event.target().unwrap();
            let text_area = target.unchecked_into::<HtmlTextAreaElement>();
            let value = text_area.value();

            new_post_text.set(value.into());
        })
    };

    html! {
            <form class={form} {onsubmit}>
                <BBTitle level={BBTitleLevel::Two}>{"Create Post"}</BBTitle>
                <textarea rows="5" cols="55" onchange={textarea_onchange}>
                </textarea>
                <button type={"submit"}>{"Submit post"}</button>
            </form>
    }
}
