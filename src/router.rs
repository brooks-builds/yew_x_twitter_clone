use crate::pages::home::Home;
use yew::{html, Html};
use yew_router::Routable;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
    }
}
