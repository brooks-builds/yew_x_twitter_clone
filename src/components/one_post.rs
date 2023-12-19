use std::ops::Deref;

use crate::{api, components::textarea::BBTextArea, router::Route, store::AppState};
use stylist::{style, yew::styled_component};
use web_sys::MouseEvent;
use yew::{classes, html, use_effect_with, use_state, AttrValue, Callback, Html, Properties};
use yew_router::prelude::Link;
use yewdux::prelude::use_store;

use crate::store::Post;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub post: Post,
    #[prop_or_default]
    pub disable_navigation: bool,
    #[prop_or_default]
    pub onedit: Callback<AttrValue>,
    #[prop_or_default]
    pub canedit: bool,
}

#[styled_component]
pub fn BBPost(props: &Props) -> Html {
    let editing_state = use_state(|| false);
    let edit_content_state = use_state(|| AttrValue::default());
    let (_state, dispatch) = use_store::<AppState>();

    {
        let edit_content_state = edit_content_state.clone();

        use_effect_with(props.post.text.clone(), move |props_text| {
            edit_content_state.set(props_text.clone());

            || {}
        });
    }

    let style = style!(
        r#"
            border: 3px solid skyblue;
            padding: 2rem;
            display: flex;
            flex-direction: column;
        "#
    )
    .unwrap();

    let link_style = style!(
        r#"
            text-decoration: none !important;
            color: black;
        "#
    )
    .unwrap();

    let navigation_hover = style!(
        r#"
            div:hover {
                background-color: gray;
            }
        "#
    )
    .unwrap();

    let right_style = style!(
        r#"
            margin-left: auto;
        "#
    )
    .unwrap();

    let edit_onclick = {
        let editing_state = editing_state.clone();

        Callback::from(move |event: MouseEvent| {
            event.stop_propagation();
            event.prevent_default();
            editing_state.set(true);
        })
    };

    let cancel_edit_onclick = {
        let editing_state = editing_state.clone();

        Callback::from(move |_event| {
            editing_state.set(false);
        })
    };

    let save_edit_onclick = {
        let editing_state = editing_state.clone();
        let edit_content_state = edit_content_state.clone();
        let dispatch = dispatch.clone();
        let id = props.post.id;

        dispatch.reduce_mut_future_callback_with(move |state, _event: MouseEvent| {
            let edit_content_state = edit_content_state.clone();
            let editing_state = editing_state.clone();

            Box::pin(async move {
                match api::edit_post(id, edit_content_state.deref().clone()).await {
                    Ok(()) => {
                        state.update_top_level_post(edit_content_state.deref().clone(), id);
                    }
                    Err(error) => {
                        gloo::console::error!("Error updating post", error.to_string());
                    }
                }

                editing_state.set(false);
            })
        })

        // Callback::from(move |_event| {
        //     onedit.emit(edit_content_state.deref().clone());
        //     editing_state.set(false);
        // })
    };

    let edit_content_onchange = {
        let edit_content_state = edit_content_state.clone();

        Callback::from(move |event: AttrValue| {
            edit_content_state.set(event);
        })
    };

    let post_html = html! {
            <div class={style.clone()}>
                {
                    if *editing_state {
                        html! {
                            <BBTextArea value={edit_content_state.deref().clone()} onchange={edit_content_onchange}/>
                        }
                    } else {
                        html! {
                            <p>{props.post.text.clone()}</p>
                        }
                    }
                }
                <p>{"likes:"} {props.post.likes}</p>
                <p>{"Responses:"} {props.post.response_count}</p>
                <div class={right_style}>
                    if *editing_state {
                        <button onclick={save_edit_onclick}>{"Save"}</button>
                        <button onclick={cancel_edit_onclick}>{"cancel"}</button>
                    } else if props.canedit {
                        <button onclick={edit_onclick}>{"Edit"}</button>
                    } else {
                        <></>
                    }
                </div>
            </div>
    };

    if props.disable_navigation {
        post_html
    } else {
        html! {
            <Link<Route> to={Route::OnePost { id: props.post.id }} classes={classes!(link_style, navigation_hover)}>
                {post_html}
            </Link<Route>>
        }
    }
}
