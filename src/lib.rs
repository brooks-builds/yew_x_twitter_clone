mod pages;
mod router;

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