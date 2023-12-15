use crate::pages::{home::Home, one_post::OnePost};
use yew::{html, Html};
use yew_router::Routable;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/posts/:id")]
    OnePost { id: u32 },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::OnePost { id } => html! { <OnePost {id} /> },
    }
}
