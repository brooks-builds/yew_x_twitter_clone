mod api;
mod components;
mod pages;
mod router;
mod store;

use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};
use yewdux::prelude::use_store;

use crate::{
    router::{switch, Route},
    store::AppState,
};

#[function_component]
pub fn App() -> Html {
    let _store = use_store::<AppState>();

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
