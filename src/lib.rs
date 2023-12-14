mod api;
mod components;
mod pages;
mod router;
mod store;

use yew::{function_component, html, use_effect, Html};
use yew_router::{BrowserRouter, Switch};
use yewdux::prelude::use_store;

use crate::{
    router::{switch, Route},
    store::AppState,
};

#[function_component]
pub fn App() -> Html {
    let (_store, dispatch) = use_store::<AppState>();

    use_effect(move || {
        let dispatch = dispatch.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let all_posts = match api::get_all_posts().await {
                Ok(posts) => posts,
                Err(error) => {
                    gloo::console::error!(
                        "There was an error getting all posts",
                        error.to_string()
                    );
                    return;
                }
            };

            dispatch.reduce_mut(move |state| {
                let posts = all_posts.into_iter().map(Into::into).collect();

                state.posts = posts;
            });
        });
        || {}
    });
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
