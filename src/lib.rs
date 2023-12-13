mod api;
mod components;
mod pages;
mod router;
mod store;

use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};

use crate::router::{switch, Route};

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
